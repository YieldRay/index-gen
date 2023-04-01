use crate::entry::Entry;
use std::fmt::Write;

fn basic_html<'a>(title: &'a str, body: &'a str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Index of /{}</title>
</head>
<body>
  <h1>Index of /{}</h1>
  {}
</body>
</html>"#,
        title, title, body
    )
}

pub fn html_for_dir(entry: &Entry, parent_dir: &str, index_file_name: &str) -> String {
    if let Entry::Dir(root_dir_name, children) = entry {
        let mut builder = String::new();
        builder.push_str("<ul>");
        if parent_dir != "." {
            writeln!(
                builder,
                "\n    <li><a href=\"../{}\">..</a></li>",
                index_file_name
            )
            .unwrap();
        }

        for e in children {
            let mut li = String::from("\n    <li>");

            let a_tag = match e {
                Entry::Dir(name, _) => {
                    format!("<a href=\"{}/{}\">{}/</a>", name, index_file_name, name)
                }
                Entry::File(name) => {
                    if name == index_file_name {
                        // exclude index_file_name
                        break;
                    } else {
                        format!("<a href=\"{}\">{}</a>", name, name)
                    }
                }
            };
            li.push_str(&a_tag);
            li.push_str("</li>");
            builder.push_str(&li)
        }

        builder.push_str("</ul>");
        builder.to_string();

        if parent_dir == "." {
            basic_html("", &builder)
        } else {
            basic_html(root_dir_name, &builder)
        }
    } else {
        panic!("Ensure Entry to be Entry::Dir !")
    }
}
