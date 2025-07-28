use crate::getentries;
use crate::hash;
use rayon::prelude::*;
// use img_hash::{HasherConfig, HashAlg};
// use image::ImageReader;
use std::{
    collections::HashMap,
    ffi::OsString,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};


pub fn find(
    dir: &OsString,
    skips: &Vec<String>,
    method: &str,
    typeoffile: String,
) -> Result<HashMap<Vec<u8>, Vec<String>>, std::io::Error> {
    let root = PathBuf::from(&dir);

    let entries = getentries::getnormalentries(&root, skips, typeoffile.clone());
    if entries.is_empty() {
        return Ok(HashMap::new());
    }

   let hash_map=match typeoffile.as_str() {
        "Document" => findDocs(&entries, method),
        "Image" => findImages(&entries, method),
        _ => {
            eprintln!("Unsupported file type: {}", typeoffile);
            return Ok(HashMap::new());
        }
    };

    Ok(hash_map)
}

fn findDocs(
    entries: &[PathBuf],
    method: &str,
) -> HashMap<Vec<u8>, Vec<String>> {
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
        let final_map = Arc::try_unwrap(hash_map)
            .expect("hashmap")
            .into_inner()
            .expect("mutex err");
        println!("{:?}",final_map);
        return final_map;
}

fn findImages(entries: &[PathBuf], method: &str) -> HashMap<Vec<u8>, Vec<String>> {
    let hash_map = Arc::new(Mutex::new(HashMap::new()));

    entries.par_iter().for_each(|path| {
        match fs::read(path) {
            Ok(bytes) => {
                let hash = hash::hash_data(method, &bytes);
                let mut map = hash_map.lock().unwrap();
                map.entry(hash)
                    .or_insert_with(Vec::new)
                    .push(path.to_string_lossy().to_string());
            }
            Err(e) => {
                eprintln!("Failed to read file {}: {}", path.display(), e);
            }
        }
    });
    let final_map=Arc::try_unwrap(hash_map)
        .expect("Multiple Arc references detected")
        .into_inner()
        .expect("Mutex poisoned");
    println!("{:?}",final_map);
    final_map
}