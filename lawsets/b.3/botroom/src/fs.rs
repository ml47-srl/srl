use std::path::Path;
use std::fs::{create_dir, remove_file, read_dir, ReadDir, File};
use std::io::{Write, Read};
use libsrl::error::SRLError;

pub fn assert_dir(path : &Path) {
	if path.is_file() {
		remove_file(path);
	}
	if !path.exists() {
		create_dir(path);
	}
}

pub fn ls(path : &Path) -> Result<Vec<String>, SRLError> {
	let rd : ReadDir = match read_dir(path) {
		Ok(x) => x,
		Err(_) => return Err(SRLError("ls".to_string(), "error opening dir".to_string()))
	};
	let mut vec : Vec<String> = Vec::new();
	for entry in rd {
		if let Ok(x) = entry {
			let os_str = x.file_name();
			let filename = os_str.to_str().unwrap();
			vec.push(filename.to_string());
		}
	}
	Ok(vec)
}

pub fn write_file(path : &Path, content : &str) -> Result<(), SRLError> {
	let mut file : File = match File::open(path) {
		Ok(x) => x,
		Err(_) => return Err(SRLError("write_file".to_string(), "error opening file".to_string()))
	};
	match file.write(content.as_bytes()) {
		Ok(x) => Ok(()),
		Err(_) => Err(SRLError("write_file".to_string(), "error writing file".to_string()))
	}
}

pub fn read_file(path : &Path) -> Result<String, SRLError> {
	let mut file : File = match File::open(path) {
		Ok(x) => x,
		Err(_) => return Err(SRLError("read_file".to_string(), "error opening file".to_string()))
	};
	let mut string = String::new();
	match file.read_to_string(&mut string) {
		Ok(x) => Ok(string),
		Err(_) => Err(SRLError("read_file".to_string(), "error reading file".to_string()))
	}
}
