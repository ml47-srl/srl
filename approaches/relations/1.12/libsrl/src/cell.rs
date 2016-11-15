use std::fmt;
use parse::*;

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
}

#[derive(PartialEq, Clone)]
pub enum Cell {
	Simple { string : String },
	Complex { cells: Vec<Cell> }
}

impl fmt::Debug for Cell {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

impl fmt::Display for Cell {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

impl Cell {
	pub fn true_cell() -> Cell {
		Cell::simple_by_str("'true'")
	}

	pub fn false_cell() -> Cell {
		Cell::simple_by_str("'false'")
	}

	pub fn equals_cell(cell1 : Cell, cell2 : Cell) -> Cell {
		Cell::complex(vec![Cell::simple_by_str("equals"), cell1, cell2])
	}

	pub fn destructure_equals_cell(equals_cell : Cell) -> Result<(Cell, Cell), String> {
		if ! equals_cell.is_valid() {
			return Err("equals_cell is invalid".to_string());
		}

		match equals_cell {
			Cell::Simple { string : _ } => return Err("equals_cell is a simple cell".to_string()),
			Cell::Complex { cells : cells_out } => {
				if cells_out.len() != 3 {
					return Err("equals_cell should have 2 arguments".to_string());
				}

				match cells_out[0] {
					Cell::Complex { cells : _ } => return Err("first argument of equals cell is complex".to_string()),
					Cell::Simple { string : ref string_out } => {
						if string_out != "equals" {
							return Err("first equals argument is not \"equals\"".to_string());
						}
					}
				}

				return Ok((cells_out[1].clone(), cells_out[2].clone()));
			}
		}
	}

	pub fn is_constant(&self) -> bool {
		match &self {
			&&Cell::Simple { string : ref string_out } => return string_out.starts_with('\'') && string_out.ends_with('\''),
			&&Cell::Complex { cells : _ } => return false
		}
	}

	pub fn is_valid(&self) -> bool {
		match &self {
			&&Cell::Simple { string : ref string_out } => {
				let mut too_many_ticks = string_out.matches('\'').count();
				if self.is_constant() {
					too_many_ticks -= 2;
				}
				return too_many_ticks == 0;
			}
			&&Cell::Complex { cells : ref cells_out } => {
				for cell in cells_out {
					if ! cell.is_valid() {
						return false;
					}
				}
				return true;
			}
		}
	}

	pub fn simple(string_arg : String) -> Cell {
		Cell::Simple { string : string_arg }
	}

	pub fn simple_by_str(string_arg : &str) -> Cell {
		Cell::Simple { string : string_arg.to_string() }
	}

	pub fn complex(cells_arg : Vec<Cell>) -> Cell {
		if cells_arg.len() < 2 {
			panic!("complex cell neeeds more than 1 argument");
		}
		Cell::Complex { cells : cells_arg }
	}

	pub fn to_string(&self) -> String { // (equals a b); a
		return match &self {
			&&Cell::Simple { string : _ } => {
				self.to_unwrapped_string()
			},
			&&Cell::Complex { cells : _ } => {
				"(".to_string() + &self.to_unwrapped_string() + ")"
			}
		}
	}

	pub fn to_unwrapped_string(&self) -> String { // equals a b | a
		match &self {
			&&Cell::Simple { string : ref string_out } => string_out.to_string(),
			&&Cell::Complex { cells : ref cells_out } => {
				let mut string = String::new();
				string.push_str(&cells_out[0].to_string());
				for cell in cells_out.iter().skip(1) {
					string.push(' ');
					string.push_str(&cell.to_string());
				}
				return string;
			}
		}
	}

	pub fn to_rule_string(&self) -> String {
		self.to_unwrapped_string() + "."
	}

	pub fn by_tokens(mut tokens : Vec<String>) -> Result<Cell, ()> {
		tokens = trim_tokens(tokens);

		// if there is only one token => return it as simple cell
		if tokens.len() == 0 {
			panic!("Cell::by_tokens(): no tokens!");
		} else if tokens.len() == 1 {
			if ! is_valid_id(&tokens[0]) {
				panic!("Cell::by_tokens(): invalid id");
			}

			return Ok(Cell::simple(tokens[0].clone()));
		} else {

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
						panic!("Cell::by_tokens(): weird invalid token='{}'", token);
					}
				}
				if parens == 0 {
					match Cell::by_tokens(tmp_tokens) {
						Ok(x) => {
							cells.push(x);
						},
						_ => panic!("Cell::by_tokens(): recursive call failed")
					}
					tmp_tokens = Vec::new();
				}
			}
			return Ok(Cell::complex(cells));
		}
	}

	pub fn by_str_tokens(tokens : Vec<&str>) -> Result<Cell, ()> {
		let mut v : Vec<String> = Vec::new();
		for token in tokens {
			v.push(token.to_string());
		}
		Cell::by_tokens(v)
	}
}

#[test]
fn test_cell_by_tokens() {
	assert_eq!(Cell::complex(vec![Cell::simple_by_str("a"), Cell::simple_by_str("b")]),
		Cell::by_str_tokens(vec!["(", "a", ")", "b"]).unwrap());
	assert_eq!(Cell::simple_by_str("wow"),
		Cell::by_str_tokens(vec!["wow"]).unwrap());
	assert_eq!(Cell::complex(vec![Cell::simple_by_str("equals"), Cell::simple_by_str("a"), Cell::simple_by_str("b")]),
		Cell::by_str_tokens(vec!["(", "equals", "a", "b", ")"]).unwrap());
}
