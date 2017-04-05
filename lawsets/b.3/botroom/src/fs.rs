use std::path::Path;
use std::fs::{create_dir, remove_file, read_dir, ReadDir};

pub fn assert_dir(path : &Path) {
	if path.is_file() {
		remove_file(path);
	}
	if !path.exists() {
		create_dir(path);
	}
}

pub fn ls(path : &Path) -> Vec<String> {
	let rd : ReadDir = read_dir(path).unwrap();
	let mut vec : Vec<String> = Vec::new();
	for entry in rd {
		if let Ok(x) = entry {
			let os_str = x.file_name();
			let filename = os_str.to_str().unwrap();
			vec.push(filename.to_string());
		}
	}
	vec
}
