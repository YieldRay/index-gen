use crate::entry::Entry;

fn basic_html<'a>(title: &'a str, body: &'a str) -> String {
    format!(
        r#"<html>
  <head>
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

        for e in children {
            let mut li = String::from("\n        <li>");

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
