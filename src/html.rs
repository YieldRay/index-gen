use crate::entry::Entry;

fn basic_html<'a>(title: &'a str, body: &'a str) -> String {
    format!(
        r#"<html>
  <head>
      <title>Index of /{}</title>
  </head>
  <body>
      {}
  </body>
</html>"#,
        title, body
    )
}

fn a_link(parent_dir: &str, file_name: &str) -> String {
    // `.` represents to root dir
    if parent_dir == "." {
        format!("<a href=\"{}\">{}</a>", file_name, file_name)
    } else {
        format!("<a href=\"{}/{}\">{}</a>", parent_dir, file_name, file_name)
    }
}

pub fn html_for_dir(entry: &Entry, parent_dir: &str, index_file_name: &str) -> String {
    if let Entry::Dir(root_dir_name, children) = entry {
        let mut builder = String::new();
        builder.push_str("<ul>");

        for e in children {
            builder.push_str("\n        <li>");

            builder.push_str(
                &(match e {
                    Entry::Dir(name, _) => {
                        a_link(parent_dir, &format!("{}/{}", name, index_file_name))
                    }
                    Entry::File(name) => a_link(parent_dir, name),
                }),
            );

            builder.push_str("</li>");
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
