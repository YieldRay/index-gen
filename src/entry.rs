use serde_json::{json, Value};
use std::{
    ffi::{OsStr, OsString},
    fs, io,
    path::Path,
};

pub enum Entry {
    Dir(String, Vec<Entry>),
    File(String),
}

impl Entry {
    pub fn new(path: &Path) -> io::Result<Entry> {
        create_entry(path, |s| s.starts_with("."))
        // TODO
    }

    pub fn print(&self, level: usize) {
        print_entry(self, level)
    }

    pub fn print_json(&self) {
        println!("{}", self.to_json())
    }

    pub fn to_json(&self) -> Value {
        entry_to_json(self)
    }
}

// fn is_dir(dir_path: &str) -> bool {
//     match fs::metadata(dir_path) {
//         Ok(md) => md.is_dir(),
//         _ => false,
//     }
// }

/// path can simply be `&Path::new(".")`  
///
/// path_filter is for filtering unwanted path
fn create_entry(path: &Path, is_omit: fn(&str) -> bool) -> io::Result<Entry> {
    let md = fs::metadata(path)?;

    let filename = path
        .file_name()
        .or(Some(&OsString::from(".")))
        .unwrap()
        .to_owned()
        .into_string()
        .unwrap();

    if md.is_file() {
        return Ok(Entry::File(filename));
    } else {
        let mut children = vec![];
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();

            if !is_omit(&child_path.file_name().unwrap().to_str().unwrap()) {
                children.push(create_entry(&child_path, is_omit)?);
            }
        }

        return Ok(Entry::Dir(filename, children));
    }
}

fn print_entry(entry: &Entry, level: usize) {
    match entry {
        Entry::File(name) => {
            println!("{}{}", " ".repeat(level * 4), name);
        }
        Entry::Dir(name, children) => {
            println!("{}{}", " ".repeat(level * 4), name);
            for child in children {
                print_entry(child, level + 1);
            }
        }
    }
}

fn entry_to_json(entry: &Entry) -> Value {
    match entry {
        Entry::File(name) => json!({
            "type": "file",
            "name": name,
        }),
        Entry::Dir(name, children) => json!({
            "type": "dir",
            "name": name,
            "children": children.iter().map(entry_to_json).collect::<Vec<_>>(),
        }),
    }
}
