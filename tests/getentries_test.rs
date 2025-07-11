use dedupx::getentries::{getentries};
use tempfile::tempdir;
use std::fs::{File};

#[test]
fn test_getentries_include(){
	let tempdir=tempdir().unwrap();
	let root=tempdir.path().to_path_buf();

	let file=root.join("test.txt");
	File::create(&file).unwrap();

	let res=getentries(&root, &vec![]);
	assert!(res.contains(&file));
}

#[test]
fn get_entries_exclude(){
	let tempdir=tempdir().unwrap();
	let root=tempdir.path().to_path_buf();

	let file=root.join("test.txt");
	File::create(&file).unwrap();

	let skips=vec![String::from(file.to_str().unwrap())];
	let res=getentries(&root, &skips);
	println!("{:?}",res);
	assert!(res.contains(&file));
}

