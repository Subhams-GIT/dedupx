use crate::hash;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    ffi::OsString,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use crate::getentries;


pub fn find(
    dir: &OsString,
    skips: &Vec<String>,
    method: &str,
    file: String,
) -> Result<HashMap<Vec<u8>, Vec<String>>, std::io::Error> {

    let root = PathBuf::from(&dir);

    
    let entries = match file.as_str() {
        "Document" => getentries::getnormalentries(&root, skips),
        "Image" => getentries::getentries(&root, skips),
        _ => Vec::new(), // fallback case, could log or error here
    };
    // println!("{:?}",entries);

    let hash_map = Arc::new(Mutex::new(HashMap::new()));

    entries.par_iter().for_each(|path| match fs::read(path) {
        Ok(content) => {
            let hash = hash::hash_data(method, &content);
            let mut map = hash_map.lock().unwrap();
            map.entry(hash)
                .or_insert_with(Vec::new)
                .push(path.to_string_lossy().to_string());
        }
        Err(e) => eprintln!("Failed to read {}: {}", path.display(), e),
    });

    let final_map = Arc::try_unwrap(hash_map).unwrap().into_inner().unwrap();

    Ok(final_map)
}
