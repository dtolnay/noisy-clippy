use crate::Span;
use syn::parse::{Error, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, LitStr, Token};

mod kw {
    syn::custom_keyword!(allow);
    syn::custom_keyword!(feature);
}

// #[allow(clippy::lint_id...)]
pub(crate) fn allow(input: ParseStream) -> Result<Vec<(String, Span)>> {
    let paths = Punctuated::<syn::Path, Token![,]>::parse_terminated(input)?;

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

// #[cfg_attr(feature = "cargo-clippy", allow(lint_id...))]
pub(crate) fn cfg_attr(input: ParseStream) -> Result<Vec<(String, Span)>> {
    input.parse::<kw::feature>()?;
    input.parse::<Token![=]>()?;
    let feature = input.parse::<LitStr>()?;
    if feature.value() != "cargo-clippy" {
        let msg = "expected feature = \"cargo-clippy\"";
        return Err(Error::new(feature.span(), msg));
    }
    input.parse::<Token![,]>()?;
    input.parse::<kw::allow>()?;

    let list;
    parenthesized!(list in input);
    input.parse::<Option<Token![,]>>()?;

    let paths = Punctuated::<syn::Path, Token![,]>::parse_terminated(&list)?;

    let mut lints = Vec::new();
    for path in paths {
        if let Some(lint_ident) = path.get_ident() {
            let span = Span {
                start: lint_ident.span().start(),
                end: lint_ident.span().end(),
            };
            lints.push((lint_ident.to_string(), span));
        }
    }

    Ok(lints)
}
