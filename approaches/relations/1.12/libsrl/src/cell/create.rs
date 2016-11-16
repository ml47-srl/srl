use parse::*;
use cell::Cell;
use cell::mani::*;

// remove optional outer parens
fn trim_tokens(mut vec : Vec<String>) -> Vec<String> {
	loop {
		let len = vec.len();

		if ! (vec[0] == "(" && vec[len-1] == ")") {
			return vec;
		}

		let mut parens = 0;

		for index in 0..len {
			let element : String = vec[index].clone();

			if element == "(" { parens += 1; }
			else if element == ")" { parens -= 1; }

			if parens == 0 && index > 0 && index < len-1 {
				return vec;
			}
		}

		vec.remove(len-1);
		vec.remove(0);
	}
}

#[test]
fn test_trim_tokens() {
	let mut tokens : Vec<String>;

	tokens = vec!["(".to_string(), "wow".to_string(), "nice".to_string(), ")".to_string(), "(".to_string(), "very".to_string(), "interesting".to_string(), ")".to_string()];
	assert_eq!(trim_tokens(tokens.clone()), tokens);

	tokens = vec!["(".to_string(), "wow".to_string(), "nice".to_string(), ")".to_string()];
	assert_eq!(trim_tokens(tokens), vec!["wow".to_string(), "nice".to_string()]);

	tokens = vec!["(".to_string(), "{".to_string(), "}".to_string(), ")".to_string()];
	assert_eq!(trim_tokens(tokens), vec!["{".to_string(), "}".to_string()]);
}

fn simple_by_trimmed_tokens(tokens : Vec<String>) -> Result<Cell, ()> {
	if ! is_valid_id(&tokens[0]) {
		panic!("simple_by_trimmed_tokens(): invalid id");
	}

	return Ok(simple(tokens[0].clone()));
}

fn complex_by_trimmed_tokens(mut tokens : Vec<String>) -> Result<Cell, ()> {
	assert!(tokens.len() > 1, format!("tokens = {:?}", tokens));

	let mut cells : Vec<Cell> = Vec::new();
	let mut tmp_tokens : Vec<String> = Vec::new();
	let mut parens = 0;

	while ! tokens.is_empty() {
		let token : String = tokens.remove(0).to_string();
		tmp_tokens.push(token.clone());
		if ! is_valid_id(&token) {
			if token == "(" {
				parens += 1;
			}
			else if token == ")" {
				parens -= 1;
			} else {
				panic!("complex_by_trimmed_tokens(): weird invalid token='{}'", token);
			}
		}
		if parens == 0 {
			match cell_by_tokens(tmp_tokens) {
				Ok(x) => {
					cells.push(x);
				},
				_ => panic!("complex_by_trimmed_tokens(): recursive call failed")
			}
			tmp_tokens = Vec::new();
		}
	}

	assert!(cells.len() > 1, format!("cells = {:?}", cells));
	return Ok(complex(cells));
}

#[test]
fn test_complex_by_trimmed_tokens() {
	assert_eq!(
		complex_by_trimmed_tokens(vec!["a".to_string(), "b".to_string()]).unwrap(),
		complex(vec![simple_by_str("a"), simple_by_str("b")])
	);
}

fn scope_by_trimmed_tokens(mut tokens : Vec<String>) -> Result<Cell, ()> {
	// cut { and }
	let len = tokens.len();

	if len < 3 { return Err(()); }
	if "}" != &tokens.remove(len-1) { return Err(()); }
	if "{" != &tokens.remove(0) { return Err(()); }

	let mut cells : Vec<Cell> = Vec::new();
	let mut tmp_tokens : Vec<String> = Vec::new();
	let mut parens = 0;

	while ! tokens.is_empty() {
		let token : String = tokens.remove(0).to_string();
		tmp_tokens.push(token.clone());
		if ! is_valid_id(&token) {
			if token == "(" {
				parens += 1;
			}
			else if token == ")" {
				parens -= 1;
			} else {
				panic!("scope_by_trimmed_tokens(): weird invalid token='{}'", token);
			}
		}
		if parens == 0 {
			match cell_by_tokens(tmp_tokens) {
				Ok(x) => {
					cells.push(x);
				},
				_ => panic!("scope_by_trimmed_tokens(): recursive call failed")
			}
			tmp_tokens = Vec::new();
		}
	}

	if cells.len() != 2 {
		return Err(());
	} else {
		return Ok(scope(cells[0].clone(), cells[1].clone()));
	}
}

#[test]
fn test_scope_by_trimmed_tokens() {
	assert_eq!(Ok(scope(simple_by_str("a"), simple_by_str("b"))), scope_by_trimmed_tokens(vec!["{".to_string(), "a".to_string(), "b".to_string(), "}".to_string()]));
	assert_eq!(Err(()), scope_by_trimmed_tokens(vec!["{".to_string(), "a".to_string(), "b".to_string(), "c".to_string(), "}".to_string()]));
	assert_eq!(Err(()), scope_by_trimmed_tokens(vec!["{".to_string(), "a".to_string(), "}".to_string()]));
	assert_eq!(Err(()), scope_by_trimmed_tokens(vec!["a".to_string(), "b".to_string(), "c".to_string(), "}".to_string()]));
	assert_eq!(Err(()), scope_by_trimmed_tokens(vec!["a".to_string(), "b".to_string(), "c".to_string()]));
	assert_eq!(Err(()), scope_by_trimmed_tokens(vec!["a".to_string()]));
	assert_eq!(Err(()), scope_by_trimmed_tokens(vec!["{".to_string(), "}".to_string()]));
}

fn var_by_trimmed_tokens(mut tokens : Vec<String>) -> Result<Cell, ()> {
	let len = tokens.len();

	if len < 3 { return Err(()); }
	if "]" != &tokens.remove(len-1) { return Err(()); }
	if "[" != &tokens.remove(0) { return Err(()); }

	return match cell_by_tokens(tokens) {
		Ok(x) => Ok(var(x)),
		Err(x) => Err(x)
	};
}

#[test]
fn test_var_by_trimmed_tokens() {
	assert_eq!(Ok(var(simple_by_str("a"))), var_by_trimmed_tokens(vec!["[".to_string(), "a".to_string(), "]".to_string()]));
	assert_eq!(Err(()), var_by_trimmed_tokens(vec!["a".to_string(), "]".to_string()]));
	assert_eq!(Err(()), var_by_trimmed_tokens(vec!["]".to_string(), "a".to_string(), "]".to_string()]));
	assert_eq!(Err(()), var_by_trimmed_tokens(vec!["[".to_string(), "]".to_string()]));
}

// trims and then calls <sub>_by_trimmed_tokens
pub fn cell_by_tokens(mut tokens : Vec<String>) -> Result<Cell, ()> {
	tokens = trim_tokens(tokens);

	let len = tokens.len();

	if len == 0 {
		panic!("cell_by_tokens(): no tokens!");
	} else if tokens.len() == 1 {
		return simple_by_trimmed_tokens(tokens);
	} else if tokens[0] == "{" && tokens[len-1] == "}" {
		return scope_by_trimmed_tokens(tokens);
	} else if tokens[0] == "[" && tokens[len-1] == "]" {
		return var_by_trimmed_tokens(tokens);
	} else {
		return complex_by_trimmed_tokens(tokens);
	}
}

pub fn cell_by_str_tokens(tokens : Vec<&str>) -> Result<Cell, ()> {
	let mut v : Vec<String> = Vec::new();
	for token in tokens {
		v.push(token.to_string());
	}
	cell_by_tokens(v)
}

#[test]
fn test_cell_by_tokens() {
	assert_eq!(complex(vec![simple_by_str("a"), simple_by_str("b")]),
		cell_by_str_tokens(vec!["(", "a", ")", "b"]).unwrap());
	assert_eq!(simple_by_str("wow"),
		cell_by_str_tokens(vec!["wow"]).unwrap());
	assert_eq!(complex(vec![simple_by_str("equals"), simple_by_str("a"), simple_by_str("b")]),
		cell_by_str_tokens(vec!["(", "equals", "a", "b", ")"]).unwrap());
}
