use clap::{Parser,ValueEnum};
use std::env;
use std::ffi::OsString;

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
        default_value_os_t = get_current_dir()  // Use default_value_os_t
    )]
    folder: OsString, 

    #[arg(short='l',long="level",help="quick-scan, full-scan, custom-scan",default_value="full-scan")]
    scan_level: ScanLevel,

    #[arg(short='s',long="speed",help="Scan speed: slow, medium, or fast",default_value="medium")]
    speed:Speed 
}
fn main() {
    // let currentDir=env::current_dir().expect("failed to get current directory");
    let args=Args::parse();

    let dir=args.folder;
    let speed=args.speed;
    let scan=args.scan_level;

    println!("{:?}",dir.into_string());
    println!("{:?}",speed);
    println!("{:?}",scan);
    
}

fn get_current_dir() -> OsString {
    env::current_dir()
        .expect("Failed to get current directory")
        .into_os_string()
}

// fn get_duplicates(hash_map: &HashMap<Vec<u8>, Vec<String>>) -> Vec<Vec<String>> {
//     hash_map.values()
//         .filter(|files| files.len() > 1)
//         .cloned()
//         .collect()
// }
/*
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


*/