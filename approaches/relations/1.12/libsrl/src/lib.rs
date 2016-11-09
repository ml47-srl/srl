mod parse;
use parse::*;

mod cell;
use cell::Cell;

use std::fs::File;
use std::io::Read;

pub struct Database {
	#[allow(dead_code)]
	rules : Vec<Cell>
}

impl Database {
	#[allow(dead_code)]
	fn by_string(string : &str) -> Database {
		match find_invalid_char(&string) {
			Some(x) => panic!("Inaccepted characters in string; char_no = {}", x),
			None => {}
		}
		let string : String = fix_whitespaces(string);
		let rule_strings = split_rules(string);
		let mut rules : Vec<Cell> = Vec::new();
		for rule_string in rule_strings {
			let mut tokens : Vec<String> = split_tokens(rule_string);

			// adding implicit parens
			if tokens.len() > 1 && tokens[0] != "(" {
				tokens.insert(0, "(".to_string());
				tokens.push(")".to_string());
			}

			match Cell::by_tokens(tokens) {
				Ok(x) => {
					rules.push(x);
				},
				Err(_) => {
					panic!("Database::by_string(): Cell::by_tokens() failed");
				}
			}
		}
		Database { rules : rules }
	}

	#[allow(dead_code)]
	fn by_filename(filename : &str) -> Database {
		let mut file : File = match File::open(filename) {
			Ok(file) => file,
			Err(_) => panic!("failed to open file")
		};
		let mut filecontent = String::new();
		match file.read_to_string(&mut filecontent) {
			Ok(_) => (),
			Err(_) => panic!("failed to read from file")
		}
		Database::by_string(&filecontent)
	}
}
