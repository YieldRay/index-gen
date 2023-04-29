use std::process::exit;

use crate::index::{html::IndexHTML, json::IndexJSON, txt::IndexTXT, ToIndexString};

pub fn select(
    prefix_name: &str,
    all: bool,
    inject: &str,
    html: bool,
    json: bool,
    txt: bool,
) -> (Box<dyn ToIndexString>, String) {
    if !html && !json && !txt {
        eprintln!("Please specify one of --html --json --txt");
        eprintln!("Use --help to see help message");
        exit(-1)
        // show clap help here
    }

    if html {
        let index_file_name = format!("{}.html", prefix_name);
        return (
            Box::new(IndexHTML {
                head: String::from(inject),
                index_file_name: index_file_name.clone(),
                all,
            }),
            index_file_name,
        );
    }

    if json {
        let index_file_name = format!("{}.json", prefix_name);
        return (Box::new(IndexJSON { all }), index_file_name);
    }

    if txt {
        let index_file_name = format!("{}.txt", prefix_name);
        return (Box::new(IndexTXT { all }), index_file_name);
    }

    // this will never happen
    panic!("")
}
