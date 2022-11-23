use crate::{Locations, SourceFile, Span};
use proc_macro2::LineColumn;
use std::collections::{BTreeMap as Map, BTreeSet as Set};
use std::path::Component;
use std::str;

pub(crate) fn render(lint_id: &str, findings: &Map<SourceFile, Locations>) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("  <meta charset=\"utf-8\">\n");
    html.push_str("  <link rel=\"stylesheet\" href=\"style.css\">\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("  <ul class=\"results-container\">\n");

    // write files containing a local suppression first
    let mut local_anchor = Some("local");
    for (source_file, locations) in findings {
        if !locations.local.is_empty() {
            let contents = &locations.contents;
            let spans = locations
                .global
                .iter()
                .chain(&locations.local)
                .copied()
                .collect();
            let anchor = local_anchor.take();
            render_file(&mut html, lint_id, source_file, contents, &spans, anchor);
        }
    }

    // files containing only global suppression
    let mut global_anchor = Some("global");
    for (source_file, locations) in findings {
        if locations.local.is_empty() {
            let contents = &locations.contents;
            let spans = locations.global.iter().copied().collect();
            let anchor = global_anchor.take();
            render_file(&mut html, lint_id, source_file, contents, &spans, anchor);
        }
    }

    html.push_str("  </ul>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");
    html
}

fn render_file(
    html: &mut String,
    lint_id: &str,
    source_file: &SourceFile,
    contents: &str,
    spans: &Set<Span>,
    anchor: Option<&str>,
) {
    let url = format!(
        "https://docs.rs/crate/{}/{}/source/{}#:~:text=clippy%3a%3a{}",
        source_file.krate,
        source_file.version,
        source_file.relative_path.display(),
        lint_id,
    );

    html.push_str("    <li class=\"result\"");
    if let Some(anchor) = anchor {
        html.push_str(" id=\"");
        html.push_str(anchor);
        html.push('"');
    }
    html.push_str(">\n");
    html.push_str("      <div class=\"result-title\">\n");
    html.push_str("        <div class=\"result-file-info\">\n");
    html.push_str("          <div class=\"result-file\"><span class=\"result-crate\">");
    html.push_str(&source_file.krate.to_string());
    html.push_str("&nbsp;");
    html.push_str(&source_file.version.to_string());
    html.push_str("</span><a href=\"");
    html.push_str(&url);
    html.push_str("\"><span class=\"result-file-directory\">/");
    let components = source_file.relative_path.components().collect::<Vec<_>>();
    let mut components = components.iter();
    while let Some(component) = components.next() {
        let component = match component {
            Component::Normal(component) => component.to_string_lossy(),
            _ => unimplemented!(),
        };
        if components.len() == 0 {
            html.push_str("</span><span class=\"result-file-name\">");
            html.push_str(&component);
        } else {
            html.push_str(&component);
            html.push_str("/<wbr>");
        }
    }
    html.push_str("</span></a></div>\n");
    html.push_str("        </div>\n");
    html.push_str("      </div>\n");
    html.push_str("      <div class=\"result-body\" onclick=\"window.open('");
    html.push_str(&url);
    html.push_str("','docsrs')\">\n");
    html.push_str("        <div class=\"result-background\"></div>\n");
    html.push_str("        <div class=\"result-details\">\n");
    html.push_str("          <table class=\"highlight-table\">\n");

    let lines: Vec<_> = contents.lines().collect();
    let mut show = vec![false; lines.len()];
    for span in spans {
        let (start, end) = (span.start.line, span.end.line);
        if start > 1 && !lines[start - 2].trim().is_empty() {
            show[start - 2] = true;
        }
        for i in start..=end {
            show[i - 1] = true;
        }
        if end < lines.len() && !lines[end].trim().is_empty() {
            show[end] = true;
        }
    }

    let mut mark = false;
    let mut spans = spans.iter();
    let mut next_span = spans.next();
    let eof = LineColumn {
        line: lines.len() + 1,
        column: 0,
    };
    for (i, line) in lines.iter().enumerate() {
        if !show[i] {
            continue;
        }

        let mut pos = LineColumn {
            line: i + 1,
            column: 0,
        };

        html.push_str("            <tr>\n");
        html.push_str("              <td><div class=\"lineno\">");
        html.push_str(&pos.line.to_string());
        html.push_str("</div></td>\n");
        html.push_str("              <td>\n");
        html.push_str("                <div class=\"highlight\">\n");
        html.push_str("                  <pre>");
        if mark {
            html.push_str("<mark>");
        }

        let mut chars = line.chars();
        while !chars.as_str().is_empty() {
            let emit_to = match next_span {
                Some(span) => {
                    if mark {
                        span.end
                    } else {
                        span.start
                    }
                }
                None => eof,
            };

            while pos < emit_to {
                let Some(ch) = chars.next() else {
                    break;
                };
                html_escape(html, ch);
                pos.column += 1;
            }

            if pos >= emit_to {
                mark ^= true;
                if mark {
                    html.push_str("<mark>");
                } else {
                    html.push_str("</mark>");
                    next_span = spans.next();
                }
            }
        }

        if mark {
            html.push_str("</mark>");
        }
        html.push_str("</pre>\n");
        if next_span.is_some() && !show.get(i + 1).unwrap_or(&true) {
            html.push_str("                  <div class=\"jump\"></div>\n");
        }
        html.push_str("                </div>\n");
        html.push_str("              </td>\n");
        html.push_str("            </tr>\n");
    }

    html.push_str("          </table>\n");
    html.push_str("        </div>\n");
    html.push_str("      </div>\n");
    html.push_str("    </li>\n");
}

fn html_escape(html: &mut String, ch: char) {
    match ch {
        '&' => html.push_str("&amp;"),
        '<' => html.push_str("&lt;"),
        '>' => html.push_str("&gt;"),
        _ => html.push(ch),
    }
}
