use dedupx::getconfig::{Args,Speed};
use clap::Parser;
#[test]

fn test_folder() {
    let current = std::env::current_dir().unwrap().into_os_string();

    let args = Args::try_parse_from(["test"]).unwrap();
    assert_eq!(args.folder, current);
}

#[test]

fn testCustomFolder(){
    use std::ffi::OsString;

	let args=Args::try_parse_from(["test", "--folder", "test2"]).unwrap();
	assert_eq!(args.folder,OsString::from("test2"));
}

#[test]
fn test_speed(){
	let args = Args::try_parse_from(["test", "--speed", "fast"]).unwrap();
    assert_eq!(args.speed, Some(Speed::Fast));
}

#[test]
fn test_speed_display_output() {
	assert_eq!(Speed::Slow.to_string(), "Slow");
	assert_eq!(Speed::Medium.to_string(), "Medium");
	assert_eq!(Speed::Fast.to_string(), "Fast");
}