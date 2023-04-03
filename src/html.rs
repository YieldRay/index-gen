use crate::entry::Entry;
use humansize::{format_size, DECIMAL};
use std::fmt::Write;

fn basic_html<'a>(dirname: &'a str, body: &'a str, head: &'a str) -> String {
    let head = if head.len() > 0 {
        format!("\n    {}", head)
    } else {
        String::from(r#"<style>li>a{display:inline-flex;min-width:20%;}<style>"#)
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

pub fn html_for_dir(entry: &Entry, parent_dir: &str, index_file_name: &str, head: &str) -> String {
    if let Entry::Dir(current_dir_name, children) = entry {
        let mut builder = String::new();
        builder.push_str("<ul>");

        // back to parent dir
        if parent_dir != "." {
            writeln!(
                builder,
                r#"{}<li><a href="../{}">..</a></li>"#,
                "\n    ", index_file_name
            )
            .unwrap();
        }

        for e in children {
            let mut li = String::from("\n    <li>");

            let a_tag = match e {
                Entry::Dir(name, _) => {
                    format!(r#"<a href="{}/{}">{}/</a>"#, name, index_file_name, name)
                }
                Entry::File(name, size) => {
                    if name == index_file_name {
                        // exclude index_file_name
                        continue;
                    } else {
                        // display file size
                        format!(
                            r#"<a href="{}">{}</a>&nbsp;&nbsp;<span>{}</span>"#,
                            name,
                            name,
                            format_size(*size, DECIMAL)
                        )
                    }
                }
            };

            li.push_str(&a_tag);
            li.push_str("</li>");
            builder.push_str(&li)
        }

        builder.push_str("\n  </ul>");
        builder.to_string();

        // println!("{}", parent_dir);

        if parent_dir == "." {
            basic_html(".", &builder, head)
        } else {
            if parent_dir == "./." {
                basic_html(&format!("./{}", current_dir_name), &builder, head)
            } else {
                basic_html(
                    &format!("{}{}", parent_dir, current_dir_name),
                    &builder,
                    head,
                )
            }
        }
    } else {
        panic!("Ensure Entry to be Entry::Dir !")
    }
}
