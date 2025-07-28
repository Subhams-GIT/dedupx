use walkdir::WalkDir;
use std::path::PathBuf;

const IMAGE: [&str; 14] = [
    ".jpg", ".jpeg", ".png", ".gif", ".webp", ".bmp", ".tif", ".tiff", ".heic", ".svg", ".ico",
    ".pdf", ".eps", ".raw",
];

pub fn getentries(root: &PathBuf, skip_set: &Vec<String>,typeoffile:String) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let rel_path = entry.path().strip_prefix(root).unwrap_or(entry.path());
            let rel_str = rel_path.to_string_lossy();
            if typeoffile=="Document".to_string() {
                return true;
            }
            if typeoffile=="Image".to_string() && IMAGE.iter().any(|ext| rel_str.ends_with(ext)) {
                return true;
            }
            if skip_set.iter().any(|skip| rel_str.starts_with(skip) || rel_str.starts_with("quarantine")) {
                return false;
            }

            if !entry.file_type().is_file() {
                return false;
            }

            false
        })
        .map(|entry| entry.into_path())
        .collect()
}


pub fn getnormalentries(root: &PathBuf, skip_set: &Vec<String>,typeoffile:String) -> Vec<PathBuf> {
    getentries(root, skip_set,typeoffile)
}
