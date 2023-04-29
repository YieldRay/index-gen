use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::index::ToIndexString;

/// Remove all index file recursively in a directory
pub fn rm_index(path: &Path, index_file_name: &str) -> (usize, usize) {
    let mut total = 0;
    let mut done = 0;

    if path.is_dir() {
        for entry in path.read_dir().expect("Failed to read directory") {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                let (done_r, total_r) = rm_index(&entry_path, index_file_name);
                done += done_r;
                total += total_r;
            }
        }
    }

    let index_path = path.join(index_file_name);
    if index_path.exists() {
        // count one dir
        total += 1;
        if let Ok(()) = fs::remove_file(&index_path) {
            // successfully remove a dir
            done += 1;
        } else {
            eprintln!("Failed to remove index file at {}", index_path.display())
        }
    }

    (done, total)
}

/// Genrate index file recursively in a directory
pub fn gen_index(
    path: &Path,
    interface: Box<dyn ToIndexString>,
    index_file_name: &str,
    overwrite: bool,
) -> (usize, usize) {
    gen_index_recursive(path, &interface, &vec![], index_file_name, overwrite)
}

fn gen_index_recursive(
    path: &Path,
    interface: &Box<dyn ToIndexString>,
    parent_segs: &Vec<&str>,
    index_file_name: &str,
    overwrite: bool,
) -> (usize, usize) {
    let mut total = 0;
    let mut done = 0;

    if path.is_dir() {
        let index_string = interface.to_index_string(path, parent_segs);
        let index_path = path.join(index_file_name);
        if !index_path.exists() || overwrite {
            // count one dir
            total += 1;

            if let Ok(mut file) = File::create(&index_path) {
                if let Ok(()) = file.write_all(index_string.as_bytes()) {
                    // successfully make a index file
                    done += 1;
                } else {
                    eprintln!("Failed to write to index file at {}", index_path.display())
                }
            } else {
                eprintln!("Failed to create index file at {}", index_path.display());
            }
        }

        for entry in path.read_dir().expect("Failed to read directory") {
            if let Ok(entry) = entry {
                // create next parent_segs, so it can call recursively
                let entry_path = entry.path();
                let entry_name = entry_path.file_name().unwrap().to_str().unwrap();
                let mut new_parent_segs = parent_segs.clone();
                new_parent_segs.push(entry_name);

                let (done_r, total_r) = gen_index_recursive(
                    &entry_path,
                    interface,
                    &new_parent_segs,
                    index_file_name,
                    overwrite,
                );
                done += done_r;
                total += total_r;
            }
        }
    }

    (done, total)
}
