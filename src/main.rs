#![allow(
    clippy::cast_possible_wrap,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_markdown,
    clippy::items_after_statements,
    clippy::let_underscore_untyped,
    clippy::match_same_arms,
    clippy::needless_lifetimes,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::uninlined_format_args,
    clippy::unwrap_or_default
)]

mod lints;
mod name;
mod parse;
mod render;

#[cfg(test)]
mod tests;

use crate::lints::{Lint, LintGroup, LintLevel};
use crate::name::Crate;
use crate::render::render;
use anyhow::Result;
use clap::Parser;
use flate2::read::GzDecoder;
use git2::{BranchType, FileMode, Repository, Signature};
use parking_lot::Mutex;
use proc_macro2::LineColumn;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use semver::Version;
use std::cmp::Reverse;
use std::collections::btree_map::{BTreeMap as Map, Entry};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::iter;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use syn::visit::Visit;
use syn::{AttrStyle, Attribute};
use tar::Archive;
use walkdir::WalkDir;

struct AttrVisitor<'a> {
    source_file: &'a SourceFile,
    contents: Arc<String>,
    findings: &'a Mutex<Findings>,
    lints: &'a Map<&'a str, &'a Lint>,
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
        let attr_path = attr.path();
        let parser = if attr_path.is_ident("allow") {
            parse::allow
        } else if attr_path.is_ident("cfg_attr") {
            parse::cfg_attr
        } else {
            return;
        };
        let Ok(lints) = attr.parse_args_with(parser) else {
            return;
        };
        if lints.is_empty() {
            return;
        }
        let mut findings = self.findings.lock();
        for (mut lint_id, span) in lints {
            lint_id = match self.lints.get(lint_id.as_str()) {
                Some(renamed_lint) => renamed_lint.id.clone(),
                None => lint_id,
            };
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

#[derive(Parser)]
#[command(version, author)]
struct Opt {
    /// Path to directory containing *.crate files.
    /// https://github.com/dtolnay/get-all-crates
    #[arg(value_name = "DIR")]
    crates_dir: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    // Download clippy lints.json to get group and level for every lint.
    let lints_vec = lints::download_lint_list()?;
    let mut lints = Map::<&str, &Lint>::new();
    for lint in &lints_vec {
        lints.insert(&lint.id, lint);
        for former_id in &lint.former_ids {
            lints.insert(former_id, lint);
        }
    }

    // Find the most recent version among .crate files with the same crate name.
    let mut crate_max_versions = Map::new();
    for entry in WalkDir::new(&opt.crates_dir) {
        let entry = entry?;
        if let Some((krate, version)) = parse_crate_file_path(&opt.crates_dir, entry.path()) {
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
            let path = reconstruct_crate_file_path(&opt.crates_dir, &krate, &version);
            if let Err(err) = parse_contents(krate, version, &path, &findings, &lints) {
                eprintln!("{}: {}", path.display(), err);
            }
        });

    // Limit rendered occurrences per file and per crate.
    const MAX_PER_FILE: usize = 5;
    const MAX_PER_CRATE: usize = 10;
    let mut findings = findings.into_inner();
    for findings in findings.values_mut() {
        let mut count_by_crate = Map::new();
        for (source_file, locations) in findings {
            locations.global.truncate(MAX_PER_FILE);
            locations
                .local
                .truncate(MAX_PER_FILE - locations.global.len());
            let n = count_by_crate.entry(&source_file.krate).or_insert(0);
            let remaining_for_crate = MAX_PER_CRATE - *n;
            locations.global.truncate(remaining_for_crate);
            locations
                .local
                .truncate(remaining_for_crate - locations.global.len());
            *n += locations.global.len() + locations.local.len();
        }
    }

    // Sort lints by how many times ignored.
    let mut findings = Vec::from_iter(&findings);
    findings.sort_by_cached_key(|(_lint_id, findings)| {
        let sum: usize = findings
            .values()
            .map(|loc| loc.global.len() + loc.local.len())
            .sum();
        Reverse(sum)
    });

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
        let mut former_group = lints::former_lint_group(lint_id);
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
        let Ok(repo) = Repository::discover(".") else {
            break;
        };
        let Ok(origin) = repo.find_remote("origin") else {
            break;
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

fn parse_crate_file_path(crates_dir: &Path, path: &Path) -> Option<(Crate, Version)> {
    let extension = path.extension()?;
    if extension != "crate" {
        return None;
    }

    let file = path.file_name()?.to_str()?;
    let first_dot = file.find('.')?;
    let separator = file[..first_dot].rfind('-')?;
    let crate_name = Crate::new(file[..separator].to_owned());
    let version = Version::parse(&file[1 + separator..file.len() - ".crate".len()]).ok()?;

    if reconstruct_crate_file_path(crates_dir, &crate_name, &version) == path {
        Some((crate_name, version))
    } else {
        None
    }
}

fn reconstruct_crate_file_path(
    crates_dir: &Path,
    crate_name: &Crate,
    version: &Version,
) -> PathBuf {
    let mut path = crates_dir.to_owned();
    let name_lower = crate_name.to_ascii_lowercase();
    match name_lower.len() {
        1 => path.push("1"),
        2 => path.push("2"),
        3 => path.extend(["3", &name_lower[..1]]),
        _ => path.extend([&name_lower[0..2], &name_lower[2..4]]),
    }
    path.push(name_lower);
    path.push(format!("{}-{}.crate", crate_name, version));
    path
}

fn parse_contents(
    krate: Crate,
    version: Version,
    path: &Path,
    findings: &Mutex<Findings>,
    lints: &Map<&str, &Lint>,
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
        let Ok(syn) = syn::parse_file(&contents) else {
            continue;
        };
        source_file.relative_path = path.iter().skip(1).collect();
        let mut visitor = AttrVisitor {
            source_file: &source_file,
            contents: Arc::new(contents),
            findings,
            lints,
        };
        visitor.visit_file(&syn);
    }
    Ok(())
}

#[test]
fn test_cli() {
    <Opt as clap::CommandFactory>::command().debug_assert();
}
