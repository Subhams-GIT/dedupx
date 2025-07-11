use walkdir::WalkDir;
use std::path::PathBuf;


pub fn getentries(root: &PathBuf, skip_set: &Vec<String>) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let rel_path = entry.path().strip_prefix(root).unwrap_or(entry.path());
            let rel_str = rel_path.to_string_lossy();

          
            if skip_set.iter().any(|skip| rel_str.starts_with(skip) || rel_str.starts_with("quarantine")) {
                return false;
            }

            
            if !entry.file_type().is_file() {
                return false;
            }


            true
        })
        .map(|entry| entry.into_path())
        .collect()
}


pub fn getnormalentries(root: &PathBuf, skip_set: &Vec<String>) -> Vec<PathBuf> {
    getentries(root, skip_set)
}
