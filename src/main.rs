#![allow(
    clippy::cast_possible_wrap,
    clippy::derive_partial_eq_without_eq,
    clippy::let_underscore_drop,
    clippy::match_same_arms,
    clippy::similar_names,
    clippy::too_many_lines
)]

mod name;
mod parse;
mod render;

use crate::name::Crate;
use crate::render::render;
use anyhow::{ensure, Result};
use flate2::read::GzDecoder;
use git2::{BranchType, FileMode, Repository, Signature};
use parking_lot::Mutex;
use proc_macro2::LineColumn;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use semver::Version;
use serde::Deserialize;
use std::cmp::Reverse;
use std::collections::btree_map::{BTreeMap as Map, Entry};
use std::env;
use std::ffi::OsStr;
use std::fmt::{self, Display};
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::iter::{self, FromIterator};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use syn::parse::Parser;
use syn::visit::Visit;
use syn::{AttrStyle, Attribute};
use tar::Archive;

struct AttrVisitor<'a> {
    source_file: &'a SourceFile,
    contents: Arc<String>,
    findings: &'a Mutex<Findings>,
}

type Findings = Map<String, Map<SourceFile, Locations>>;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct SourceFile {
    krate: Crate,
    version: Version,
    relative_path: PathBuf,
}

struct Locations {
    contents: Arc<String>,
    global: Vec<Span>,
    local: Vec<Span>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Span {
    start: LineColumn,
    end: LineColumn,
}

// Find all lint level attributes and count how many times each Clippy lint is
// allowed.
impl<'ast, 'a> Visit<'ast> for AttrVisitor<'a> {
    fn visit_attribute(&mut self, attr: &'ast Attribute) {
        let parser = if attr.path.is_ident("allow") {
            parse::allow
        } else if attr.path.is_ident("cfg_attr") {
            parse::cfg_attr
        } else {
            return;
        };
        let lints = match parser.parse2(attr.tokens.clone()) {
            Ok(lints) => lints,
            Err(_) => return,
        };
        if lints.is_empty() {
            return;
        }
        let mut findings = self.findings.lock();
        for (lint_id, span) in lints {
            let locations = findings
                .entry(lint_id)
                .or_insert_with(Map::new)
                .entry(self.source_file.clone())
                .or_insert_with(|| Locations {
                    contents: Arc::clone(&self.contents),
                    global: Vec::new(),
                    local: Vec::new(),
                });
            match attr.style {
                AttrStyle::Outer => locations.local.push(span),
                AttrStyle::Inner(_) => locations.global.push(span),
            }
        }
    }
}

#[derive(Deserialize)]
struct Lint {
    id: String,
    group: LintGroup,
    level: LintLevel,
}

#[derive(Deserialize, PartialEq, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
enum LintGroup {
    Cargo,
    Complexity,
    Correctness,
    Deprecated,
    Nursery,
    Pedantic,
    Perf,
    Restriction,
    Style,
    Suspicious,
    Unknown,
}

#[derive(Deserialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
enum LintLevel {
    Allow,
    Warn,
    Deny,
    None,
}

impl Display for LintGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("{:?}", self).to_lowercase())
    }
}

fn main() -> Result<()> {
    // Argument is path to a directory containing *.crate files. See these links
    // for how to bulk download crate sources without violating crawlers policy:
    // https://twitter.com/m_ou_se/status/1433085053056262144
    // https://www.pietroalbini.org/blog/downloading-crates-io/
    let mut args = env::args_os();
    ensure!(args.len() == 2);
    let _arg0 = args.next().unwrap();
    let crates_dir = args.next().unwrap();

    // Find the most recent version among .crate files with the same crate name.
    let mut crate_max_versions = Map::new();
    for entry in fs::read_dir(&crates_dir)? {
        let entry = entry?;
        if let Some((krate, version)) = parse_filename(&entry.file_name()) {
            match crate_max_versions.entry(krate) {
                Entry::Vacant(entry) => {
                    entry.insert(version);
                }
                Entry::Occupied(mut entry) => {
                    if version > *entry.get() {
                        entry.insert(version);
                    }
                }
            }
        }
    }

    ThreadPoolBuilder::new()
        .stack_size(20 * 1024 * 1024)
        .build_global()
        .unwrap();

    // Parse .crate files in parallel on rayon thread pool.
    let findings = Mutex::new(Map::new());
    crate_max_versions
        .into_par_iter()
        .for_each(|(krate, version)| {
            let filename = format!("{}-{}.crate", krate, version);
            let path = Path::new(&crates_dir).join(filename);
            if let Err(err) = parse_contents(krate, version, &path, &findings) {
                eprintln!("{}: {}", path.display(), err);
            }
        });

    // Sort lints by how many times ignored.
    let findings = findings.into_inner();
    let mut findings = Vec::from_iter(&findings);
    findings.sort_by_cached_key(|(_lint_id, findings)| {
        let sum: usize = findings
            .values()
            .map(|loc| loc.global.len() + loc.local.len())
            .sum();
        Reverse(sum)
    });

    // Download clippy lints.json to get group and level for every lint.
    let req = reqwest::blocking::get("https://rust-lang.github.io/rust-clippy/master/lints.json")?;
    let lints: Vec<Lint> = req.json()?;
    let lints: Map<&str, &Lint> = lints.iter().map(|lint| (lint.id.as_str(), lint)).collect();

    // Print markdown table of results.
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = writeln!(stdout, "local | global | lint name | category");
    let _ = writeln!(stdout, "--- | --- | --- | ---");
    let site = "https://dtolnay.github.io/noisy-clippy";
    for (lint_id, findings) in &findings {
        let (group, level) = match lints.get(lint_id.as_str()) {
            Some(lint) => (lint.group, lint.level),
            None => (LintGroup::Unknown, LintLevel::None),
        };
        let allowed = level == LintLevel::Allow;
        let _ = write!(stdout, "{}", if allowed { "~*" } else { "" });
        let local: usize = findings.values().map(|loc| loc.local.len()).sum();
        let _ = if local == 0 {
            write!(stdout, "{}", local)
        } else {
            write!(stdout, "[{}]({}/{}.html#local)", local, site, lint_id)
        };
        let _ = write!(stdout, "{}", if allowed { "*~" } else { "" });
        let _ = write!(stdout, " | ");
        let _ = write!(stdout, "{}", if allowed { "~*" } else { "" });
        let global: usize = findings.values().map(|loc| loc.global.len()).sum();
        let _ = if global == 0 {
            write!(stdout, "{}", global)
        } else {
            write!(stdout, "[{}]({}/{}.html#global)", global, site, lint_id)
        };
        let _ = write!(stdout, "{}", if allowed { "*~" } else { "" });
        let _ = write!(stdout, " | ");
        let _ = write!(stdout, "{}", if allowed { "~*" } else { "**" });
        let clippy_index_html = "https://rust-lang.github.io/rust-clippy/master/index.html";
        let _ = write!(stdout, "[{1}]({0}#{1})", clippy_index_html, lint_id);
        let _ = write!(stdout, "{}", if allowed { "*~" } else { "**" });
        let _ = write!(stdout, " | ");
        let mut former_group = former_lint_group(lint_id);
        if former_group == Some(group) {
            former_group = None;
        }
        if let Some(former_group) = former_group {
            let _ = write!(stdout, "~*{}*~ ", former_group);
        }
        let _ = write!(stdout, "{}", group);
        let _ = writeln!(stdout);
    }

    for () in iter::once(()) {
        let repo = match Repository::discover(".") {
            Ok(repo) => repo,
            Err(_) => break,
        };
        let origin = match repo.find_remote("origin") {
            Ok(origin) => origin,
            Err(_) => break,
        };
        if origin.url_bytes() != b"gh:dtolnay/noisy-clippy" {
            break;
        }

        let tree_entries = None;
        let mut builder = repo.treebuilder(tree_entries)?;
        let filemode = u32::from(FileMode::Blob) as i32;
        for (lint_id, findings) in &findings {
            let html = render(lint_id, findings);
            let filename = format!("{}.html", lint_id);
            let oid = repo.blob(html.as_bytes())?;
            builder.insert(filename, oid, filemode)?;
        }

        let oid = repo.blob(include_bytes!("style.css"))?;
        builder.insert("style.css", oid, filemode)?;
        let oid = builder.write()?;

        let tree = repo.find_tree(oid)?;
        if let Ok(diff) = repo
            .find_branch("gh-pages", BranchType::Local)
            .and_then(|old_branch| old_branch.get().peel_to_tree())
            .and_then(|old_tree| {
                let diff_options = None;
                repo.diff_tree_to_tree(Some(&old_tree), Some(&tree), diff_options)
            })
        {
            if diff.deltas().len() == 0 {
                break;
            }
        }

        let update_ref = None;
        let signature = Signature::now("David Tolnay", "dtolnay@gmail.com")?;
        let msg = "Update gh-pages";
        let parents = &[];
        let oid = repo.commit(update_ref, &signature, &signature, msg, &tree, parents)?;

        let branch_name = "gh-pages";
        let commit = repo.find_commit(oid)?;
        let force = true;
        repo.branch(branch_name, &commit, force)?;
    }

    Ok(())
}

fn parse_filename(file: &OsStr) -> Option<(Crate, Version)> {
    let extension = Path::new(file).extension()?;
    if extension != "crate" {
        return None;
    }

    let file = file.to_str()?;
    let first_dot = file.find('.')?;
    let separator = file[..first_dot].rfind('-')?;
    let crate_name = Crate::new(file[..separator].to_owned());
    let version = Version::parse(&file[1 + separator..file.len() - ".crate".len()]).ok()?;
    Some((crate_name, version))
}

fn parse_contents(
    krate: Crate,
    version: Version,
    path: &Path,
    findings: &Mutex<Findings>,
) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tar = GzDecoder::new(reader);
    let mut archive = Archive::new(tar);
    let mut source_file = SourceFile {
        krate,
        version,
        relative_path: PathBuf::new(),
    };
    for entry in archive.entries()? {
        let mut entry = entry?;
        if entry.size() > 10 * 1024 * 1024 {
            continue;
        }
        let path = entry.path()?;
        if path.extension() != Some(OsStr::new("rs")) {
            continue;
        }
        let path = path.into_owned();
        let mut contents = String::new();
        if entry.read_to_string(&mut contents).is_err() {
            break;
        }
        let syn = match syn::parse_file(&contents) {
            Ok(syn) => syn,
            Err(_) => continue,
        };
        source_file.relative_path = path.iter().skip(1).collect();
        let mut visitor = AttrVisitor {
            source_file: &source_file,
            contents: Arc::new(contents),
            findings,
        };
        visitor.visit_file(&syn);
    }
    Ok(())
}

fn former_lint_group(lint_id: &str) -> Option<LintGroup> {
    match lint_id {
        // @dtolnay
        "cognitive_complexity" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/5428
        "implicit_hasher" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5411
        "inefficient_to_string" => Some(LintGroup::Perf), // https://github.com/rust-lang/rust-clippy/pull/5412
        "integer_division" => Some(LintGroup::Pedantic), // https://github.com/rust-lang/rust-clippy/pull/4210
        "large_digit_groups" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/3479
        "let_unit_value" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5409
        "manual_map" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/6796
        "match_bool" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5408
        "needless_pass_by_value" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/3439
        "new_ret_no_self" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5420
        "nonstandard_macro_braces" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/7424
        "option_if_let_else" => Some(LintGroup::Pedantic), // https://github.com/rust-lang/rust-clippy/pull/7568
        "option_option" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/5401
        "rc_buffer" => Some(LintGroup::Perf), // https://github.com/rust-lang/rust-clippy/pull/6128
        "string_lit_as_bytes" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/6117
        "trivial_regex" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/6696
        "trivially_copy_pass_by_ref" => Some(LintGroup::Perf), // https://github.com/rust-lang/rust-clippy/pull/5410
        "unnested_or_patterns" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/5705
        "unreadable_literal" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5419
        "unsafe_vector_initialization" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/3478
        "useless_let_if_seq" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5599

        // others
        "await_holding_lock" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/6354
        "await_holding_refcell_ref" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/6354
        "borrow_interior_mutable_const" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/6098
        "cast_ptr_alignment" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/5667
        "declare_interior_mutable_const" => Some(LintGroup::Correctness), // https://github.com/rust-lang/rust-clippy/pull/6098
        "match_wild_err_arm" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/5622
        "range_plus_one" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/5057
        "suspicious_operation_groupings" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/7266
        "unnecessary_wraps" => Some(LintGroup::Complexity), // https://github.com/rust-lang/rust-clippy/pull/6765
        "verbose_bit_mask" => Some(LintGroup::Style), // https://github.com/rust-lang/rust-clippy/pull/6036

        _ => None,
    }
}
