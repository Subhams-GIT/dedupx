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

}


pub fn get_config()-> (OsString,Speed){
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



	return (dir,speed)

}

fn get_current_dir() -> OsString {
    env::current_dir()
        .expect("Failed to get current directory")
        .into_os_string()
}
