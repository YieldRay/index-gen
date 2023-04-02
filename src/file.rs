use std::{fs, path::PathBuf};

use crate::{entry::Entry, html::html_for_dir};

fn path_to_unix_string(path: &PathBuf) -> String {
    path.display()
        .to_string()
        .replace(r#"\"#, "/")
        .replace("/./", "/")
    // .replacen("./", "", 1)
}

/// generate index_file_name for parent_dir/entry(if is a dir)   
///
/// for current dir, parent_dir should be `.`  
///
/// force: force wirte file even if the file already exists  
pub fn gen_index(entry: &Entry, parent_dir: &str, index_file_name: &str, force: bool) -> usize {
    let mut success_count = 0;

    if let Entry::Dir(dirname, children) = entry {
        let dirpath = PathBuf::from_iter(vec![parent_dir, dirname]);
        println!(
            "Generating {} for {}/",
            index_file_name,
            path_to_unix_string(&dirpath)
        );

        let filepath = PathBuf::from_iter(vec![parent_dir, dirname, index_file_name]);

        if filepath.is_file() && !force {
            eprintln!(
                "Index file {} already exists (use -f to force write)",
                path_to_unix_string(&filepath)
            );
        } else {
            if let Err(e) = fs::write(filepath, html_for_dir(entry, parent_dir, index_file_name)) {
                eprintln!("{}", e);
                panic!();
            } else {
                success_count += 1;
            }
        }

        for child in children {
            let pathbuf = &PathBuf::from_iter(vec![parent_dir, dirname]);
            success_count += gen_index(child, &path_to_unix_string(pathbuf), index_file_name, force)
        }
    }
    success_count
}
