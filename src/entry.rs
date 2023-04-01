use serde_json::{json, Value};
use std::{
    ffi::OsString,
    fmt::{self, Write},
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

    pub fn print(&self) {
        print!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        entry_to_string(self, 4, 0)
    }

    pub fn print_json(&self) {
        println!("{}", self.to_json())
    }

    pub fn to_json(&self) -> Value {
        entry_to_json(self)
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

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

fn entry_to_string(entry: &Entry, indent: usize, depth: usize) -> String {
    let mut sb = String::new();
    let space = " ".repeat(indent * depth);
    match entry {
        Entry::File(name) => {
            writeln!(sb, "{}{}", space, name).unwrap();
        }
        Entry::Dir(name, children) => {
            println!("{}{}/", space, name);
            for child in children {
                let str = entry_to_string(child, indent, depth + 1);
                write!(sb, "{}", str).unwrap();
            }
        }
    }
    sb
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
