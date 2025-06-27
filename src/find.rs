use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::fs;
use std::process;
use walkdir::WalkDir;
use std::collections::HashMap;


pub fn find(dir: OsString)->Result<HashMap<Vec<u8>, Vec<String>>, std::io::Error>  {
    let mut hash_map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
    for entry in WalkDir::new(&dir).into_iter().filter_map(Result::ok) {
		// let entry=entry?;
        let path = entry.path();

		let metadata=match fs::metadata(path){
			Ok(mt)=>mt,
			Err(e) => {
                eprintln!("Error reading metadata for {}: {}", path.display(), e);
                continue;
            }
		};

		if metadata.len()==0 {
			// println!("{:?}",path);
			continue;
		}

        else if path.is_file() {
            let content = fs::read(path).unwrap_or_else(|_| {
                eprintln!("Error reading file: {}", path.display());
                process::exit(1);
            });

            let mut hasher = Sha256::new();
            hasher.update(&content);
            let hash = hasher.finalize().to_vec();

            hash_map
                .entry(hash)
                .or_insert_with(Vec::new)
                .push(path.to_string_lossy().to_string());
        }
    }
	return Ok(hash_map);
}
