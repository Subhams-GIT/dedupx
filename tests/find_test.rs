use dedupx::find;
use tempfile::tempdir;
use std::ffi::OsString;
use std::fs::{File};
use std::io::Write;
use std::path::PathBuf;

fn write_file(path: &PathBuf, content: &str) {
	let mut file = File::create(path).expect("Failed to create file");
	file.write_all(content.as_bytes()).expect("Failed to write content");
}

#[test]
fn test_duplicates(){
	let temp_dir = tempdir().unwrap();
    let root = temp_dir.path().to_path_buf();

	let file1=root.join("a.txt");
	let file3=root.join("c.txt");

	write_file(&file1, "hello");
	write_file(&file3, "hello");

	let skips=vec![];
	let res=find::find(&OsString::from(&root), &skips, "sha256").unwrap();

	for path in res.values(){
		if path.len()>1{
			println!("{:?}",path);
			assert!(path.iter().any(|p| p.contains("a.txt") || p.contains("c.txt")));
		}
	}

}