use crate::Span;
use syn::parse::{ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Token};

// #[allow(clippy::lint_id)]
pub(crate) fn allow(input: ParseStream) -> Result<Vec<(String, Span)>> {
    let content;
    parenthesized!(content in input);

    let paths = Punctuated::<syn::Path, Token![,]>::parse_terminated(&content)?;

    let mut lints = Vec::new();
    for path in paths {
        if path.segments.len() == 2 && path.segments[0].ident == "clippy" {
            let clippy_ident = &path.segments[0].ident;
            let lint_ident = &path.segments[1].ident;
            let span = Span {
                start: clippy_ident.span().start(),
                end: lint_ident.span().end(),
            };
            lints.push((lint_ident.to_string(), span));
        }
    }

    Ok(lints)
}
