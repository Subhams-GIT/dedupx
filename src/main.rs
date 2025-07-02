use clap::{Parser,ValueEnum};
use std::{env};
use std::ffi::OsString;
use std::collections::HashMap;
mod find;
use inquire::{Select};
use std::fmt;
mod read;
mod hash;
#[derive(Debug,Clone,ValueEnum,PartialEq)]
enum Speed{
    Slow,
    Medium,
    Fast
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Speed::Slow => "Slow",
            Speed::Medium => "Medium",
            Speed::Fast => "Fast",
        };
        write!(f, "{}", label)
    }
}

#[derive(Parser)]
struct Args{
   #[arg(
        short = 'f',
        long = "folder",
        help = "Directory to scan (default: current directory)",
        default_value_os_t = get_current_dir()  
    )]
    folder: OsString, 


    #[arg(short = 's', long = "speed", help = "Scan speed")]
    speed: Option<Speed>,

}


fn main(){
    let args=Args::parse();
    let dir=args.folder;
    let skips = read::skips().unwrap_or_else(|_| {
        println!("error in reading");
        Vec::new()
    });
    let speed = args.speed.unwrap_or_else(|| {
        Select::new("Choose speed:", vec![
            Speed::Slow,
            Speed::Medium,
            Speed::Fast
        ])
        .prompt()
        .expect("Prompt failed")
    });

   if speed.to_string()=="Slow" {
    let hash_map=find::find(dir,skips,"sha256");
    let dup_files = get_duplicates(&hash_map);
    println!("{:?}",dup_files);
   } else if speed.to_string() == "Medium" {
       let hash_map=find::find(dir,skips,"blake3");
       let dup_files = get_duplicates(&hash_map);
        println!("{:?}",dup_files);
   } else{
    let hash_map=find::find(dir,skips,"xxh3");
    let dup_files = get_duplicates(&hash_map);
     println!("{:?}",dup_files);
   } 
    
}

fn get_current_dir() -> OsString {
    env::current_dir()
        .expect("Failed to get current directory")
        .into_os_string()
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
