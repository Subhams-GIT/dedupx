use std::{env, process};
use sha2::{Sha256, Digest};
use std::fs;
use walkdir::WalkDir;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        process::exit(1);
    }


    let dir = &args[1];
    
    let mut hash_map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let content = fs::read(path).unwrap_or_else(|_| {
                eprintln!("Error reading file: {}", path.display());
                process::exit(1);
            });

            let mut hasher = Sha256::new();
            hasher.update(&content);
            let hash = hasher.finalize().to_vec();

            hash_map.entry(hash)
                .or_insert_with(Vec::new)
                .push(path.to_string_lossy().to_string());
        }
    }

    let dup_files = get_duplicates(&hash_map);
    println!("Duplicate files:");
    for group in dup_files {
        println!("{:?}", group);
    }
}


fn get_duplicates(hash_map: &HashMap<Vec<u8>, Vec<String>>) -> Vec<Vec<String>> {
    hash_map.values()
        .filter(|files| files.len() > 1)
        .cloned()
        .collect()
}
