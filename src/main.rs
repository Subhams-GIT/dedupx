use clap::{Parser,ValueEnum};
use std::env;
use std::ffi::OsString;
use std::collections::HashMap;
mod find;

#[derive(Debug, Clone, ValueEnum)]
enum ScanLevel {
    QuickScan,
    FullScan,
    CustomScan,
}

#[derive(Debug,Clone,ValueEnum)]
enum Speed{
    Slow,
    Medium,
    Fast
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

    #[arg(short='l',long="level",help="quick-scan, full-scan, custom-scan",default_value="full-scan")]
    scan_level: ScanLevel,

    #[arg(short='s',long="speed",help="Scan speed: slow, medium, or fast",default_value="medium")]
    speed:Speed 


}


fn main() {
    // let items=vec!["enter and find duplicate files","exit"];

    // let selection = MultiSelect::new()
    //     .with_prompt("What do you choose?")
    //     .items(&items)
    //     .interact()
    //     .unwrap();

    // println!("{:?}",selection);
    let args=Args::parse();


    let dir=args.folder;
    // let speed=args.speed;
    // let scan=args.scan_level;

    let hash_map=find::find(dir);

    let dup_files = get_duplicates(&hash_map);
    println!("Duplicate files:");
    for group in dup_files {
        println!("{:?}", group);
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
