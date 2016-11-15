static VALID_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ _=().\n\t'";
static VALID_ID_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_='";

pub fn is_valid_id(string : &str) -> bool {
	for chr in string.chars() {
		if ! VALID_ID_CHARS.contains(chr) {
			return false;
		}
	}
	return true;
}

pub fn fix_whitespaces(string : &str) -> String {
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
			Some(x) => { string.remove(x); },
			None => break
		}
	}
	// "equals a b. " => "equals a b."
	match string.chars().rev().nth(0) {
		Some(x) => {
			if x == ' ' {
				string.pop();
			}
		}
		None => {}
	}
	// " equals a b." => "equals a b."
	match string.chars().nth(0) {
		Some(x) => {
			if x == ' ' {
				string = string.chars().skip(1).collect();
			}
		}
		None => {}
	}
	// "( " => "("
	loop {
		match string.find("( ") {
			Some(x) => { string.remove(x+1); }
			None => break
		}
	}
	// " )" => ")"
	loop {
		match string.find(" )") {
			Some(x) => { string.remove(x); }
			None => break
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

pub fn find_invalid_char(string : &str) -> Option<i32> {
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

pub fn split_rules(string : String) -> Vec<String> {
	let mut vec : Vec<String> = Vec::new();
	let mut string : String = string.to_string();

	if string.is_empty() {
		return Vec::new();
	}

	if ! string.ends_with(".") {
		panic!("string does not end with '.'");
	}

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
	assert!(split_rules("".to_string()).is_empty());
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

#[test]
#[should_panic]
fn test_split_rules4() {
	split_rules("good".to_string());
}

pub fn split_tokens(mut string : String) -> Vec<String> {
	let mut tokens : Vec<String> = Vec::new();

	let mut tmp_string = String::new();
	while string.len() > 0 {
		match string.remove(0) {
			'(' => {
				if ! tmp_string.is_empty() {
					tokens.push(tmp_string);
					tmp_string = String::new();
				}
				tokens.push("(".to_string());
			},
			')' => {
				if ! tmp_string.is_empty() {
					tokens.push(tmp_string);
					tmp_string = String::new();
				}
				tokens.push(")".to_string());
			},
			' ' => {
				if ! tmp_string.is_empty() {
					tokens.push(tmp_string);
					tmp_string = String::new();
				}
			},
			chr => {
				if VALID_ID_CHARS.contains(chr) {
					tmp_string.push(chr);
				} else {
					panic!("char is not valid for use in ID: {}", chr);
				}
			},
		}
	}
	if ! tmp_string.is_empty() {
		tokens.push(tmp_string);
	}
	tokens
}

#[test]
fn test_split_tokens() {
	assert_eq!(split_tokens("(wow good)".to_string()), vec!["(".to_string(), "wow".to_string(), "good".to_string(), ")".to_string()]);
	assert_eq!(split_tokens("wow".to_string()), vec!["wow".to_string()]);
}
