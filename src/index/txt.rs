use std::fmt::Write;
use std::path::Path;

use super::ToIndexString;

pub struct IndexTXT {
    pub all: bool,
}

impl ToIndexString for IndexTXT {
    fn to_index_string(&self, path: &Path, _parent_segs: &Vec<&str>) -> String {
        txt_for_dir(path, self.all)
    }
}

/// Genrate Txt for a Path object
pub fn txt_for_dir(path: &Path, all: bool) -> String {
    let mut txt = String::new();

    if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();

                // ignore files that starts with .
                if !all && name.starts_with(".") {
                    continue;
                }

                let md = entry.metadata().unwrap();
                if md.is_dir() {
                    writeln!(txt, "{}/", name).unwrap();
                } else if md.is_file() {
                    writeln!(txt, "{}", name).unwrap();
                }
            }
        }
    }

    txt
}
