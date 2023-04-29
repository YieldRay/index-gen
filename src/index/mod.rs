use std::path::Path;

pub(crate) mod html;
pub(crate) mod json;
pub(crate) mod txt;

// the only API we need to achieve is to
// generate a INDEX_FILE for a directory
// as the INDEX_FILE is pure String
// the only interface simply returns String
pub trait ToIndexString {
    fn to_index_string(&self, path: &Path, parent_segs: &Vec<&str>) -> String;
}
