use std::collections::HashMap;
mod delete;
mod find;
mod getconfig;
mod getentries;
mod hash;
mod read;
use crate::delete::Quarantine;
use inquire::Select;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let (dir, speed, typeofile) = getconfig::get_config();
    let skips = read::skips().unwrap_or_else(|_| {
        println!("error in reading");
        Vec::new()
    });

    let duplicate_files: Vec<Vec<String>> = match speed.to_string().as_str() {
        "Slow" => {
            let hash_map = find::find(&dir, &skips, "sha256", typeofile.to_string());
            let dup_files = get_duplicates(&hash_map);
            dup_files
        }
        "Medium" => {
            let hash_map = find::find(&dir, &skips, "blake3", typeofile.to_string());
            let dup_files = get_duplicates(&hash_map);
            dup_files
        }
        _ => {
            let hash_map = find::find(&dir, &skips, "xxh3", typeofile.to_string());
            println!("Found {:?} entries", hash_map);
            let dup_files = get_duplicates(&hash_map);
            dup_files
        }
    };
    println!("Found {:?} duplicate files", duplicate_files);
    let option = Select::new(
        "Choose whether you want to delete or quarantine",
        vec!["delete", "Quarantine"],
    )
    .prompt()
    .expect("Prompt failed");

    let mut quarantine = Quarantine::qdir()?;

    let dup_paths: Vec<Vec<PathBuf>> = duplicate_files
        .into_iter()
        .map(|group| group.into_iter().map(PathBuf::from).collect())
        .collect();

    match quarantine.quarantine_duplicates(dup_paths, option) {
        Ok(quarantined) => println!("successfully quarantined files: {:?}", quarantined),
        Err(e) => eprintln!("Failed to quarantine files: {}", e),
    }

    Ok(())
}

fn get_duplicates(hash_map: &Result<HashMap<Vec<u8>, Vec<String>>>) -> Vec<Vec<String>> {
    match hash_map {
        Ok(map) => map
            .values()
            .filter(|files| files.len() > 1)
            .cloned()
            .collect(),
        Err(_) => Vec::new(),
    }
}
