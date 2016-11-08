mod cell;
use cell::Cell;
use std::fs::File;
use std::io::Read;

static VALID_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ _=().\n\t";

pub struct Database {
	rules : Vec<Cell>
}

fn fix_whitespaces(string : &str) -> String {
	// '\t', '\n'  => ' '
	let mut string : String = string.chars().map(|c| {
		match c {
			'\t' | '\n' | ' ' => ' ',
			x => x
		}
	}).collect();
	// "  " => " "
	loop {
		match string.find("  ") {
			Some(x) => { string.remove(x); }
			None => { break; }
		}
	}
	// "equals a b. " => "equals a b."
	loop {
		match string.chars().rev().nth(0) {
			Some(x) => {
				if x == ' ' {
					string.pop();
					break;
				}
			}
			None => { panic!("fix_whitespaces(): string is empty"); }
		}
	}
	string
}

#[test]
fn test_fix_whitespaces() {
	assert_eq!(fix_whitespaces("wow.
	 x 
	 "), "wow. x");
}

fn find_invalid_char(string : &str) -> Option<i32> {
	let mut x = 0;
	for i in string.chars() {
		if ! VALID_CHARS.contains(i) {
			return Some(x);
		}
		x += 1;
	}
	None
}

#[test]
fn test_find_invalid_char() {
	assert_eq!(find_invalid_char("wowäsdf"), Some(3));
}

impl Database {
	fn by_string(string : &str) -> Database {
		match find_invalid_char(&string) {
			Some(x) => { panic!("Inaccepted characters in string; char_no = {}", x); }
			None => ()
		}
		let string = fix_whitespaces(string);
		let rules : Vec<Cell> = Vec::new();
		Database { rules : rules }
	}

	fn by_filename(filename : &str) -> Database {
		let mut file : File = match File::open(filename) {
			Ok(file) => file,
			Err(_) => { panic!("failed to open file"); }
		};
		let mut filecontent = String::new();
		match file.read_to_string(&mut filecontent) {
			Ok(_) => (),
			Err(_) => { panic!("failed to read from file"); }
		}
		Database::by_string(&filecontent)
	}
}
