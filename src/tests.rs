use crate::name::Crate;
use crate::{AttrVisitor, SourceFile};
use parking_lot::Mutex;
use quote::quote;
use semver::Version;
use std::collections::BTreeMap as Map;
use std::path::PathBuf;
use std::sync::Arc;
use syn::visit::Visit;
use syn::File;

#[test]
fn test_attr_visitor() {
    let input = quote! {
        #![allow(clippy::asdf)]

        fn main() {
            #[cfg_attr(feature = "cargo-clippy", allow(jkl))]
            let _;
        }
    };

    let findings = Mutex::new(Map::new());
    let mut visitor = AttrVisitor {
        source_file: &SourceFile {
            krate: Crate::new("test".to_owned()),
            version: Version::new(0, 0, 0),
            relative_path: PathBuf::from("src/lib.rs"),
        },
        contents: Arc::new(input.to_string()),
        findings: &findings,
    };

    let file: File = syn::parse_str(&visitor.contents).unwrap();
    visitor.visit_file(&file);

    let findings = findings.into_inner();
    assert_eq!(findings["asdf"].len(), 1);
    assert_eq!(findings["jkl"].len(), 1);
}
