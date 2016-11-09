mod cell;
use cell::Cell;
use std::fs::File;
use std::io::Read;

static VALID_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ _=().\n\t";
static VALID_ID_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_=";

pub struct Database {
	rules : Vec<Cell>
}

fn is_valid_id(string : &str) -> bool {
	for chr in string.chars() {
		if ! VALID_ID_CHARS.contains(chr) {
			return false;
		}
	}
	return true;
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
	// "( " => "("
	loop {
		match string.find("( ") {
			Some(x) => { string.remove(x+1); }
			None => { break; }
		}
	}
	// " )" => ")"
	loop {
		match string.find(" )") {
			Some(x) => { string.remove(x); }
			None => { break; }
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
	assert_eq!(fix_whitespaces(" \n  ( abc  )"), "(abc)");
	assert_eq!(fix_whitespaces(" abc def ()  \t ( abc  )"), "abc def () (abc)");
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
				let (new_rule_string, tmp_string) = string_clone.split_at(x+1);
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
		}
	}
	vec
}

#[test]
fn test_split_rules() {
	assert_eq!(split_rules("wow.nice.good.".to_string()), vec!["wow".to_string(), "nice".to_string(), "good".to_string()]);
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

fn split_tokens(string : &str) -> Vec<String> {
	let mut tokens : Vec<String> = Vec::new();

	let mut string : String = string.to_string();

	loop {
		match string.chars().nth(0) {
			Some('(') => {
				string = string.chars().skip(1).collect();
				tokens.push("(".to_string());
			},
			Some(')') => {
				string = string.chars().skip(1).collect();
				tokens.push(")".to_string());
			},
			Some(_) => {
				let mut tmp_string : String = String::new();
				for chr in string.clone().chars() {
					if chr == ' ' {
						string = string.chars().skip(tmp_string.len() + 1).collect();
						tokens.push(tmp_string);
						break;
					} else if chr == ')' {
						string = string.chars().skip(tmp_string.len()).collect();
						tokens.push(tmp_string);
						break;
					} else if VALID_ID_CHARS.contains(chr) {
						tmp_string.push(chr);
					} else {
						panic!("char is not valid for use in ID: {}", chr);
					}
				}
			},
			None => { break; }
		}
	}
	tokens
}

#[test]
fn test_split_tokens() {
	assert_eq!(split_tokens("(wow good)"), vec!["(".to_string(), "wow".to_string(), "good".to_string(), ")".to_string()]);
}

impl Database {
	#[allow(dead_code)]
	fn by_string(string : &str) -> Database {
		match find_invalid_char(&string) {
			Some(x) => { panic!("Inaccepted characters in string; char_no = {}", x); }
			None => ()
		}
		let string = fix_whitespaces(string);
		let rule_strings = split_rules(string);
		let mut rules : Vec<Cell> = Vec::new();
		for rule_string in rule_strings {
			let tokens : Vec<String> = split_tokens(&rule_string);
			match Cell::by_tokens(&tokens) {
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
