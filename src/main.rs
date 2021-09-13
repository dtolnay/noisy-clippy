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
    let mut args = env::args_os();
    ensure!(args.len() == 2);
    let _arg0 = args.next().unwrap();
    let crates_dir = args.next().unwrap();

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

    let visitor = AttrVisitor::default();
    crate_max_versions.par_iter().for_each(|(krate, version)| {
        let filename = format!("{}-{}.crate", krate, version);
        let path = Path::new(&crates_dir).join(filename);
        if let Err(err) = parse_contents(&path, &visitor) {
            eprintln!("{}: {}", path.display(), err);
        }
    });

    let clippy_allows = visitor.clippy_allows.into_inner();
    let mut clippy_allows = Vec::from_iter(&clippy_allows);
    clippy_allows.sort_by(|(lname, lcount), (rname, rcount)| {
        let lcount = lcount.global + lcount.local;
        let rcount = rcount.global + rcount.local;
        lcount.cmp(&rcount).reverse().then_with(|| lname.cmp(rname))
    });

    let req = reqwest::blocking::get("https://rust-lang.github.io/rust-clippy/master/lints.json")?;
    let lints: Vec<Lint> = req.json()?;
    let lints: Map<&str, &Lint> = lints.iter().map(|lint| (lint.id.as_str(), lint)).collect();

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
