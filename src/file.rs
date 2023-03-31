use std::{fs, path::PathBuf};

use crate::{entry::Entry, html::html_for_dir};

// TODO: --force is not implemented yet
pub fn gen_index(entry: &Entry, parent_dir: &str, index_file_name: &str) {
    if let Entry::Dir(dirname, children) = entry {
        println!(
            "Generating {} for {}/{}",
            index_file_name, parent_dir, dirname
        );
        if let Err(e) = fs::write(
            PathBuf::from_iter(vec![parent_dir, dirname, index_file_name]),
            html_for_dir(entry, parent_dir, index_file_name),
        ) {
            eprintln!("{}", e);
            panic!();
        }

        for child in children {
            gen_index(
                child,
                &PathBuf::from_iter(vec![parent_dir, dirname])
                    .as_os_str()
                    .to_string_lossy()
                    .replace(r#"\"#, "/"),
                index_file_name,
            )
        }
    }
}
