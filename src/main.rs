use anyhow::{ensure, Result};
use flate2::read::GzDecoder;
use parking_lot::Mutex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use semver::Version;
use serde::Deserialize;
use std::collections::btree_map::{BTreeMap as Map, Entry};
use std::env;
use std::ffi::OsStr;
use std::fmt::{self, Display};
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::iter::FromIterator;
use std::path::Path;
use syn::parse::{ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::{parenthesized, AttrStyle, Attribute, Token};
use tar::Archive;

#[derive(Default)]
struct AttrVisitor {
    clippy_allows: Mutex<Map<String, Ignores>>,
}

#[derive(Default)]
struct Ignores {
    global: usize,
    local: usize,
}

// Find all #[allow(...)] and #![allow(...)] attributes and count how many times
// each Clippy lint is allowed.
impl<'ast> Visit<'ast> for &AttrVisitor {
    fn visit_attribute(&mut self, attr: &'ast Attribute) {
        if !attr.path.is_ident("allow") {
            return;
        }
        let parse_allow_attribute = |input: ParseStream| {
            let content;
            parenthesized!(content in input);
            Punctuated::<syn::Path, Token![,]>::parse_terminated(&content)
        };
        let paths = match parse_allow_attribute.parse2(attr.tokens.clone()) {
            Ok(paths) => paths,
            Err(_) => return,
        };
        for path in paths {
            if path.segments.len() == 2 && path.segments[0].ident == "clippy" {
                let lint = path.segments.last().unwrap().ident.to_string();
                let mut clippy_allows = self.clippy_allows.lock();
                let ignores = clippy_allows.entry(lint).or_default();
                match attr.style {
                    AttrStyle::Outer => ignores.local += 1,
                    AttrStyle::Inner(_) => ignores.global += 1,
                }
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
    let visitor = AttrVisitor::default();
    crate_max_versions.par_iter().for_each(|(krate, version)| {
        let filename = format!("{}-{}.crate", krate, version);
        let path = Path::new(&crates_dir).join(filename);
        if let Err(err) = parse_contents(&path, &visitor) {
            eprintln!("{}: {}", path.display(), err);
        }
    });

    // Sort lints by how many times ignored.
    let clippy_allows = visitor.clippy_allows.into_inner();
    let mut clippy_allows = Vec::from_iter(&clippy_allows);
    clippy_allows.sort_by(|(lname, lcount), (rname, rcount)| {
        let lcount = lcount.global + lcount.local;
        let rcount = rcount.global + rcount.local;
        lcount.cmp(&rcount).reverse().then_with(|| lname.cmp(rname))
    });

    // Download clippy lints.json to get group and level for every lint.
    let req = reqwest::blocking::get("https://rust-lang.github.io/rust-clippy/master/lints.json")?;
    let lints: Vec<Lint> = req.json()?;
    let lints: Map<&str, &Lint> = lints.iter().map(|lint| (lint.id.as_str(), lint)).collect();

    // Print markdown table of results.
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = writeln!(stdout, "global | local | lint name | category");
    let _ = writeln!(stdout, "--- | --- | --- | ---");
    for (lint_id, count) in &clippy_allows {
        let (group, level) = match lints.get(lint_id.as_str()) {
            Some(lint) => (lint.group, lint.level),
            None => (LintGroup::Unknown, LintLevel::None),
        };
        let allowed = level == LintLevel::Allow;
        let _ = write!(stdout, "{}", if allowed { "~*" } else { "" });
        let _ = write!(stdout, "{}", count.global);
        let _ = write!(stdout, "{}", if allowed { "*~" } else { "" });
        let _ = write!(stdout, " | ");
        let _ = write!(stdout, "{}", if allowed { "~*" } else { "" });
        let _ = write!(stdout, "{}", count.local);
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

    Ok(())
}

fn parse_filename(file: &OsStr) -> Option<(String, Version)> {
    let extension = Path::new(file).extension()?;
    if extension != "crate" {
        return None;
    }

    let file = file.to_str()?;
    let first_dot = file.find('.')?;
    let separator = file[..first_dot].rfind('-')?;
    let crate_name = file[..separator].to_owned();
    let version = Version::parse(&file[1 + separator..file.len() - ".crate".len()]).ok()?;
    Some((crate_name, version))
}

fn parse_contents(path: &Path, mut visitor: &AttrVisitor) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tar = GzDecoder::new(reader);
    let mut archive = Archive::new(tar);
    for entry in archive.entries()? {
        let mut entry = entry?;
        if entry.size() > 10 * 1024 * 1024 {
            continue;
        }
        let path = entry.path()?;
        if path.extension() != Some(OsStr::new("rs")) {
            continue;
        }
        let mut contents = String::new();
        if entry.read_to_string(&mut contents).is_err() {
            break;
        }
        let syn = match syn::parse_file(&contents) {
            Ok(syn) => syn,
            Err(_) => continue,
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
