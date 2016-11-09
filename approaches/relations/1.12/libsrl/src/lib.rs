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
				}
				break;
			}
			None => { panic!("fix_whitespaces(): string is empty"); }
		}
	}
	// " equals a b." => "equals a b."
	loop {
		match string.chars().nth(0) {
			Some(x) => {
				if x == ' ' {
					string = string.chars().skip(1).collect();
				}
				break;
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
	 "), "wow. x".to_string());
	assert_eq!(fix_whitespaces("abc   "), "abc");
	assert_eq!(fix_whitespaces("   abc"), "abc");
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

fn split_rules(string : String) -> Vec<String> {
	let mut vec : Vec<String> = Vec::new();
	let mut string : String = string.to_string();
	loop {
		match string.find(".") {
			Some(x) => {
				if x == 0 {
					panic!("invalid '.'-expression at beginning");
				}
				let string_clone = string.clone();
				let (mut new_rule_string, mut tmp_string) = string_clone.split_at(x+1);
				string = tmp_string.to_string();
				let mut new_rule_string : String = new_rule_string.to_string();
				new_rule_string.pop();
				if ! new_rule_string.is_empty() {
					vec.push(new_rule_string);
				}
			}
			None => {
				if ! string.is_empty() {
					vec.push(string);
				}
				break;
			}
		};
	}
	vec
}

#[test]
fn test_split_rules() {
	let mut x : Vec<String> = Vec::new();
	x.push("wow".to_string());
	x.push("nice".to_string());
	x.push("good".to_string());
	assert_eq!(split_rules("wow.nice.good.".to_string()), x);
}

#[test]
#[should_panic]
fn test_split_rules2() {
	split_rules(".nice.good.".to_string());
}

#[test]
#[should_panic]
fn test_split_rules3() {
	split_rules("nice..good.".to_string());
}

impl Database {
	fn by_string(string : &str) -> Database {
		match find_invalid_char(&string) {
			Some(x) => { panic!("Inaccepted characters in string; char_no = {}", x); }
			None => ()
		}
		let string = fix_whitespaces(string);
		let rule_strings = split_rules(string);
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
