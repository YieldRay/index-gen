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
pub fn gen_index(
    entry: &Entry,
    parent_dir: &PathBuf,
    index_file_name: &str,
    force: bool,
    css: &str,
) -> usize {
    let mut success_count = 0;

    if let Entry::Dir(dirname, children) = entry {
        // create dirpath by parent_dir/dirname/
        let mut dirpath = parent_dir.clone();
        dirpath.push(format!("{}/", dirname));

        println!(
            "Generating {} for {}",
            index_file_name,
            path_to_unix_string(&dirpath)
        );

        // create filepath by parent_dir/dirname/index_file_name
        let mut filepath = dirpath.clone();
        filepath.push(index_file_name);

        // write index_file
        if filepath.is_file() && !force {
            eprintln!(
                "Index file {} already exists (use -f to force write)",
                path_to_unix_string(&filepath)
            );
        } else {
            if let Err(e) = fs::write(
                filepath,
                html_for_dir(
                    entry,
                    &path_to_unix_string(parent_dir),
                    index_file_name,
                    css,
                ),
            ) {
                eprintln!("{}", e);
                panic!();
            } else {
                success_count += 1;
            }
        }

        for child in children {
            success_count += gen_index(child, &dirpath, index_file_name, force, css)
        }
    }
    success_count
}

pub fn rm_index(entry: &Entry, parent_dir: &PathBuf, index_file_name: &str) -> usize {
    let mut success_count = 0;

    if let Entry::Dir(name, children) = entry {
        // create dirpath by parent_dir/name
        let mut dirpath = parent_dir.clone();
        dirpath.push(format!("{}/", name));

        // create filepath by parent_dir/name/index_file_name
        let mut filepath = dirpath.clone();
        filepath.push(index_file_name);

        // remove index in this dir
        if filepath.is_file() {
            if let Ok(()) = fs::remove_file(&filepath) {
                success_count += 1;
                println!("Removed index file {}", path_to_unix_string(&filepath));
            } else {
                eprintln!(
                    "Failed to removed index file {}",
                    path_to_unix_string(&filepath)
                );
            }
        }

        // remove index in sub dir
        for child in children {
            success_count += rm_index(child, &dirpath, index_file_name);
        }
    }

    success_count
}
