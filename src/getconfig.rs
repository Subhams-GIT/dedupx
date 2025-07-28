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
pub enum Type{
    Document,
    Image,
    Other
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Type::Document => "Document",
            Type::Image => "Image",
            Type::Other => "Other",
        };
        write!(f, "{}", label)
    }
}


#[derive(Parser)]
pub struct Args{
   #[arg(
        short = 'f',
        long = "folder",
        help = "Directory to scan (default: current directory)",
        default_value_os_t = get_current_dir()  
    )]
    pub folder: OsString, 

    #[arg(short = 's', long = "speed", help = "Scan speed")]
    pub speed: Option<Speed>,
    
    #[arg(short = 't', long = "type", help = "Type of file")]
    pub file: Option<Type>,

}


pub fn get_config()-> (OsString,Speed,Type){
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

    let typeoffile = args.file.unwrap_or_else(|| {
        Select::new("Choose Type of file :", vec![
            Type::Document,
            Type::Image,
            Type::Other,
        ])
        .prompt()
        .expect("Prompt failed")
    });

	return (dir,speed,typeoffile)

}

fn get_current_dir() -> OsString {
    env::current_dir()
        .expect("Failed to get current directory")
        .into_os_string()
}
