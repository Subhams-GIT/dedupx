use walkdir::WalkDir;
use std::path::PathBuf;

pub fn getentries(root:&PathBuf,skip_set:&Vec<String>)->Vec<PathBuf>{
	let image_extensions = [
        "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp", "svg", "ico", "heic", "heif",
    ];
	let entries: Vec<PathBuf> = WalkDir::new(&root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let rel_path = entry.path().strip_prefix(&root).unwrap_or(entry.path());
            let rel_str = rel_path.to_string_lossy();
            let should_include = !skip_set.iter().any(|skip| rel_str.starts_with(skip))
                && entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext_str| image_extensions.contains(&ext_str.to_lowercase().as_str()))
                    .unwrap_or(false);
                return should_include;
        })
        .map(|entry| entry.into_path())
        .collect();
	return entries
}

pub fn getnormalentries(root:&PathBuf,skip_set:&Vec<String>)-> Vec<PathBuf>{
	let entries: Vec<PathBuf> = WalkDir::new(&root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let rel_path = entry.path().strip_prefix(&root).unwrap_or(entry.path());
            let rel_str = rel_path.to_string_lossy();
            let should_include = !skip_set.iter().any(|skip| rel_str.starts_with(skip))
                && entry.file_type().is_file();
                return should_include;
        })
        .map(|entry| entry.into_path())
        .collect();
	return entries;
}

