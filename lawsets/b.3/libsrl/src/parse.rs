use error::SRLError;

pub static VALID_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_=0123456789'(){} \n\t.";

pub static SIMPLE_CELL_FILL_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
pub static VAR_FILL_CHARS : &'static str = "0123456789";

pub static LPARENS : &'static str = "{(";
pub static RPARENS : &'static str = "})";
pub static PARENS : &'static str = "{(})";

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
	for paren in LPARENS.chars() {
		loop {
			let mut pattern : String = String::new();
			pattern.push(paren);
			pattern.push(' ');

			match string.find(&pattern) {
				Some(x) => { string.remove(x+1); }
				None => break
			}
		}
	}
	// " )" => ")"
	for paren in RPARENS.chars() {
		loop {
			let mut pattern : String = " ".to_string();
			pattern.push(paren);
			match string.find(&pattern) {
				Some(x) => { string.remove(x); }
				None => break
			}
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

// splits a database string, where fix_whitespaces was already called into rule-strings
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

// splits string into tokens, fix_whitespaces has to be called prior. Defined behaviour only for chars in VALID_CHARS without \n \t and .
pub fn split_tokens(mut string : String) -> Result<Vec<String>, SRLError> {
	let mut tokens : Vec<String> = Vec::new();

	#[allow(non_camel_case_types)]
	enum State { NONE, VAR, SIMPLE, CONST, EQ, AFTER_CELL };
	let mut state : State = State::NONE;

	let mut tmp_string : String = String::new();

	loop {
		if string.is_empty() {
			break;
		}
		match (string.remove(0), &state) {
			(x, &State::NONE) => {
				if PARENS.contains(x) {
					tokens.push(x.to_string())
				} else if x == ' ' {
					/* nothing */
				} else if x == '\'' {
					state = State::CONST;
					tmp_string.push('\'');
				} else if x == '=' {
					state = State::EQ;
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					tmp_string.push(x);
					state = State::SIMPLE;
				} else if VAR_FILL_CHARS.contains(x) {
					tmp_string.push(x);
					state = State::VAR;
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in NONE state", x)))
				}
			}
			(x, &State::VAR) => {
				if PARENS.contains(x) {
					tokens.push(tmp_string);
					tokens.push(x.to_string());
					tmp_string = String::new();
					state = State::NONE;
				} else if x == ' ' {
					tokens.push(tmp_string);
					tmp_string = String::new();
					state = State::NONE;
				} else if x == '\'' {
					return Err(SRLError("split_tokens".to_string(), format!("trying to close var {} with constant tick ' ", tmp_string)));
				} else if x == '=' {
					return Err(SRLError("split_tokens".to_string(), format!("trying to close var {} with =", tmp_string)));
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("trying to close var {} with {}", tmp_string, x)));
				} else if VAR_FILL_CHARS.contains(x) {
					tmp_string.push(x);
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in VAR state", x)))
				}
			}
			(x, &State::SIMPLE) => {
				if PARENS.contains(x) {
					tokens.push(tmp_string);
					tokens.push(x.to_string());
					tmp_string = String::new();
					state = State::NONE;
				} else if x == ' ' {
					tokens.push(tmp_string);
					tmp_string = String::new();
					state = State::NONE;
				} else if x == '\'' {
					return Err(SRLError("split_tokens".to_string(), format!("trying to end simple cell \"{}\" with tick '", tmp_string)));
				} else if x == '=' {
					return Err(SRLError("split_tokens".to_string(), format!("trying to end simple cell \"{}\" with =", tmp_string)));
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					tmp_string.push(x);
				} else if VAR_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("trying to end simple cell \"{}\" with {}", tmp_string, x)));
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in SIMPLE state", x)))
				}
			}
			(x, &State::CONST) => {
				if PARENS.contains(x) {
					tokens.push(tmp_string);
					tokens.push(x.to_string());
					tmp_string = String::new();
					state = State::NONE;
				} else if x == ' ' {
					tokens.push(tmp_string);
					tmp_string = String::new();
					state = State::NONE;
				} else if x == '\'' {
					tmp_string.push('\'');
					tokens.push(tmp_string);
					tmp_string = String::new();
					state = State::AFTER_CELL;
				} else if x == '=' {
					return Err(SRLError("split_tokens".to_string(), format!("trying to end const cell \"{}\" with =", tmp_string)));
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					tmp_string.push(x);
				} else if VAR_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("trying to end const cell \"{}\" with {}", tmp_string, x)));
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in CONST state", x)));
				}
			},
			(x, &State::EQ) => {
				if PARENS.contains(x) {
					tokens.push("=".to_string());
					tokens.push(x.to_string());
					state = State::NONE;
				} else if x == ' ' {
					tokens.push("=".to_string());
					state = State::NONE;
				} else if x == '\'' {
					return Err(SRLError("split_tokens".to_string(), "trying to put ' after '='".to_string()));
				} else if x == '=' {
					return Err(SRLError("split_tokens".to_string(), "trying to put '=' after '='".to_string()));
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("trying to put '{}' after '='", x)));
				} else if VAR_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("trying to put '{}' after '='", x)));
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in EQ state", x)));
				}
			}
			(x, &State::AFTER_CELL) => {
				if PARENS.contains(x) {
					tokens.push(x.to_string());
					state = State::NONE;
				} else if x == ' ' {
					state = State::NONE;
				} else if x == '\'' {
					return Err(SRLError("split_tokens".to_string(), format!("reading '{}' directly after cell end", x)));
				} else if x == '=' {
					return Err(SRLError("split_tokens".to_string(), "reading '=' directly after cell end".to_string()));
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("reading '{}' directly after cell end", x)));
				} else if VAR_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("reading '{}' directly after cell end", x)));
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in AFTER_CELL state", x)))
				}
			}
		}
	}

	Ok(tokens)
}

#[test]
fn test_split_tokens() {
	assert_eq!(split_tokens("(wow good)".to_string()).unwrap(), vec!["(".to_string(), "wow".to_string(), "good".to_string(), ")".to_string()]);
	assert_eq!(split_tokens("wow".to_string()).unwrap(), vec!["wow".to_string()]);
	assert_eq!(split_tokens("{x}".to_string()).unwrap(), vec!["{".to_string(), "x".to_string(), "}".to_string()]);
}

pub fn is_simple_token(token : &str) -> bool {
	match token.chars().next() {
		Some(chr) => return SIMPLE_CELL_FILL_CHARS.contains(chr) || chr == '=' || chr == '\'',
		None => false
	}
}

pub fn is_var_token(token: &str) -> bool {
	match token.chars().next() {
		Some(chr) => return VAR_FILL_CHARS.contains(chr),
		None => false
	}
}
