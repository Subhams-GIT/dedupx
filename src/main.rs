use std::collections::HashMap;
mod find;
mod read;
mod hash;
mod getconfig;
mod getentries;

fn main(){

    let (dir,speed,typeof_file)=getconfig::get_config();
    let skips = read::skips().unwrap_or_else(|_| {
        println!("error in reading");
        Vec::new()
    });
   
    match typeof_file.to_string().as_str() {
        "Image" => {
            let hash_map = find::find(&dir, &skips, "Image", typeof_file.to_string());
            let dup_files = get_duplicates(&hash_map);
            println!("{:?}", dup_files);
        }
        _ => {
            match speed.to_string().as_str() {
                "Slow" => {
                    let hash_map = find::find(&dir, &skips, "sha256", typeof_file.to_string());
                    let dup_files = get_duplicates(&hash_map);
                    println!("{:?}", dup_files);
                }
                "Medium" => {
                    let hash_map = find::find(&dir, &skips, "blake3", typeof_file.to_string());
                    let dup_files = get_duplicates(&hash_map);
                    println!("{:?}", dup_files);
                }
                _ => {
                    let hash_map = find::find(&dir, &skips, "xxh3", typeof_file.to_string());
                    let dup_files = get_duplicates(&hash_map);
                    println!("duplicate files{:?}", dup_files);
                }
            }
        }
    } 
    
}


fn get_duplicates(hash_map: &Result<HashMap<Vec<u8>, Vec<String>>, std::io::Error>) -> Vec<Vec<String>> {
    match hash_map {
        Ok(map) => {
            map.values()
                .filter(|files| files.len() > 1)
                .cloned()
                .collect()
        },
        Err(_) => Vec::new(), 
    }
}
