use std::{fs, path::PathBuf};

use crate::{entry::Entry, html::html_for_dir};

fn is_file(path: &PathBuf) -> bool {
    if let Ok(md) = fs::metadata(path) {
        return md.is_file();
    }
    false
}

/// generate index_file_name for parent_dir/entry(if is a dir)
///
/// force: force wirte file even if the file already exists
pub fn gen_index(entry: &Entry, parent_dir: &str, index_file_name: &str, force: bool) {
    if let Entry::Dir(dirname, children) = entry {
        println!(
            "Generating {} for {}/{}",
            index_file_name, parent_dir, dirname
        );
        let filepath = PathBuf::from_iter(vec![parent_dir, dirname, index_file_name]);
        if is_file(&filepath) && !force {
            eprintln!(
                "Index file {} already exists (use -f to force write)",
                filepath.to_string_lossy()
            );
        } else {
            if let Err(e) = fs::write(filepath, html_for_dir(entry, parent_dir, index_file_name)) {
                eprintln!("{}", e);
                panic!();
            }
        }

        for child in children {
            gen_index(
                child,
                &PathBuf::from_iter(vec![parent_dir, dirname])
                    .as_os_str()
                    .to_string_lossy()
                    .replace(r#"\"#, "/"),
                index_file_name,
                force,
            )
        }
    }
}
