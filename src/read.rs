use std::fs::File;
use std::io::{self, BufRead};

pub fn skips() -> io::Result<Vec<String>> {
    let path = ".gitignore";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut skip_list = Vec::new();

    for (_i, line) in reader.lines().enumerate() {
        let mut line = line?; 
        
        
        if line.trim_end().ends_with('/') {
            line.pop();
            skip_list.push(line);
        }
    }

    Ok(skip_list)
}
