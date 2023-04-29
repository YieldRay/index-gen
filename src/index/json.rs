use std::path::Path;

use serde_json::json;

use super::ToIndexString;

pub struct IndexJSON {
    pub all: bool,
}

impl ToIndexString for IndexJSON {
    fn to_index_string(&self, path: &Path, _parent_segs: &Vec<&str>) -> String {
        json_for_dir(path, self.all)
    }
}

/// Genrate JSON for a Path object
pub fn json_for_dir(path: &Path, all: bool) -> String {
    let mut arr = Vec::new();

    if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();

                // ignore files that starts with .
                if !all && name.starts_with(".") {
                    continue;
                }

                let md = entry.metadata().unwrap();
                let size = md.len();
                let r#type = if md.is_dir() { "folder" } else { "file" };
                arr.push(json!({"type":r#type, "name":name, "size":size}));
            }
        }
    }

    json!(arr).to_string()
}
