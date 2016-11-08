mod cell;
use cell::Cell;
use std::fs::File;
use std::io::Read;

static VALID_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ _=()";

fn validate_string(string : &str) -> bool {
	for i in string.chars() {
		if ! VALID_CHARS.contains(i) {
			return false;
		}
	}
	true
}

pub struct Database {
	rules : Vec<Cell>
}

impl Database {
	fn by_filename(filename : &str) -> Database {
		let rules : Vec<Cell> = Vec::new();
		let mut file : File = match File::open(filename) {
			Ok(file) => file,
			Err(_) => { panic!("failed to open file"); }
		};
		let mut filecontent = String::new();
		match file.read_to_string(&mut filecontent) {
			Ok(_) => (),
			Err(_) => { panic!("failed to read from file"); }
		}
		if ! validate_string(&filecontent) {
			panic!("Inaccepted characters in file");
		}
		
		Database { rules : rules }
	}
}
