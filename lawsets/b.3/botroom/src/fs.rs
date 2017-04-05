use std::path::Path;
use std::fs::create_dir;
use std::fs::remove_file;

pub fn assert_dir(path : &Path) {
	if path.is_file() {
		remove_file(path);
	}
	if !path.exists() {
		create_dir(path);
	}
}
