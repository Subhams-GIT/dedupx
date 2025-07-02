use rayon::prelude::*;
use std::{
    collections::HashMap,
    ffi::OsString,
    fs,
    path::{PathBuf},
    sync::{Arc, Mutex},
};
use walkdir::WalkDir;
use crate::hash;
pub fn find(
    dir: OsString,
    skips: Vec<String>,
    method:&str
) -> Result<HashMap<Vec<u8>, Vec<String>>, std::io::Error> {
   
    let root = PathBuf::from(&dir);
    let skip_set = skips; 


    let entries: Vec<PathBuf> = WalkDir::new(&root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let rel_path = entry.path().strip_prefix(&root).unwrap_or(entry.path());
            let rel_str = rel_path.to_string_lossy();
            !skip_set.iter().any(|skip| rel_str.starts_with(skip))
                && entry.file_type().is_file()
        })
        .map(|entry| entry.into_path())
        .collect();

       
    let hash_map = Arc::new(Mutex::new(HashMap::new()));

    
    entries.par_iter().for_each(|path| {
        match fs::read(path) {
            Ok(content) => {
                
                let hash =hash::hash_data(method, &content); 
                let mut map = hash_map.lock().unwrap();
                map.entry(hash)
                    .or_insert_with(Vec::new)
                    .push(path.to_string_lossy().to_string());
            }
            Err(e) => eprintln!("Failed to read {}: {}", path.display(), e),
        }
    });

    
    let final_map = Arc::try_unwrap(hash_map)
        .unwrap()
        .into_inner()
        .unwrap();

    Ok(final_map)
}
