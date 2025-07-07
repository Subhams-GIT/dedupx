use clap::{Parser,ValueEnum};
use std::{env};
use std::ffi::OsString;
use inquire::{Select};
use std::fmt;

#[derive(Debug,Clone,ValueEnum,PartialEq)]
pub enum Speed{
    Slow,
    Medium,
    Fast
}

#[derive(Debug,Clone,ValueEnum,PartialEq)]
pub enum TypeofFile{
    Document,
    Image,
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


impl fmt::Display for TypeofFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            TypeofFile::Document => "Document",
            TypeofFile::Image => "Image",
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

    #[arg(short = 't', long = "Type", help = "Enter Type of File ")]
    typef: Option<TypeofFile>,

}


pub fn get_config()-> (OsString,Speed,TypeofFile){
    let args=Args::parse();
    let dir=args.folder;

    let speed = args.speed.unwrap_or_else(|| {
        Select::new("Choose speed:", vec![
            Speed::Slow,
            Speed::Medium,
            Speed::Fast
        ])
        .prompt()
        .expect("Prompt failed")
    });


    let typeof_file = args.typef.unwrap_or_else(|| {
        Select::new("Choose type of file you want to scan :", vec![
            TypeofFile::Document,
            TypeofFile::Image,
        ])
        .prompt()
        .expect("Prompt failed")
    });

	return (dir,speed,typeof_file);
}

fn get_current_dir() -> OsString {
    env::current_dir()
        .expect("Failed to get current directory")
        .into_os_string()
}
