use humansize::{format_size, DECIMAL};
use std::{fmt::Write, path::Path};

use super::ToIndexString;

pub struct IndexHTML {
    pub head: String,
    pub index_file_name: String,
    pub all: bool,
}

impl ToIndexString for IndexHTML {
    fn to_index_string(&self, path: &Path, parent_segs: &Vec<&str>) -> String {
        html_for_dir(
            path,
            &self.index_file_name,
            parent_segs,
            &self.head,
            self.all,
        )
    }
}

/// Generate HTML for a Path object
fn html_for_dir(
    path: &Path,
    index_file_name: &str,
    parent_segs: &Vec<&str>,
    head: &str,
    all: bool,
) -> String {
    let mut body = String::from("<ul>\n");
    if parent_segs.len() > 0 {
        // anchor for backto parent dir
        writeln!(
            body,
            r#"    <li><a href="../{}">..</a></li>"#,
            index_file_name
        )
        .unwrap();
    }

    if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                // find dir and file in this dir
                let name = entry.file_name().into_string().unwrap();

                // ignore files that starts with .
                if !all && name.starts_with(".") {
                    continue;
                }

                let path = entry.path();
                if path.is_dir() {
                    // dir
                    writeln!(
                        body,
                        r#"    <li><a href="./{}/{}">{}/</a></li>"#,
                        name, index_file_name, name
                    )
                    .unwrap()
                } else if path.is_file() {
                    // file
                    let size = path.metadata().unwrap().len();
                    writeln!(
                        body,
                        r#"    <li><a href="{}">{}</a>&nbsp;&nbsp;<span>{}</span></li>"#,
                        name,
                        name,
                        format_size(size, DECIMAL)
                    )
                    .unwrap();
                }
            }
        }
    }

    body.push_str("</ul>");
    basic_html(
        &format!(
            "{}",
            if parent_segs.len() == 0 {
                String::from(".")
            } else {
                parent_segs.join("/")
            }
        ),
        &body,
        head,
    )
}

/// Generate a HTML document container
fn basic_html<'a>(dirname: &'a str, body: &'a str, head: &'a str) -> String {
    let head = if head.len() > 0 {
        format!("\n    {}", head)
    } else {
        String::from(r#"<style>li>a{display:inline-flex;min-width:20%;}</style>"#)
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Index of {}/</title>{}
</head>
<body>
  <h1>Index of {}/</h1>
  {}
</body>
</html>"#,
        dirname, head, dirname, body
    )
}
