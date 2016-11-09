use std::fmt;

#[derive(PartialEq)]
pub enum Cell {
	SimpleCell { string : String },
	ComplexCell { cells: Vec<Cell> } 
}

impl fmt::Debug for Cell {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

impl Cell {
	fn simple(string_arg : String) -> Cell {
		Cell::SimpleCell { string : string_arg }
	}

	fn complex(cells_arg : Vec<Cell>) -> Cell {
		Cell::ComplexCell { cells : cells_arg }
	}

	pub fn to_string(&self) -> String {
		match &self {
			&&Cell::SimpleCell { string : ref x } => x.to_string(),
			&&Cell::ComplexCell { cells : ref x } => {
				let mut s = String::from("(");
				for i in x {
					let tmp_string : String = " ".to_string() + &i.to_string() + " ";
					s.push_str(&tmp_string);
				}
				s.push(')');
				s
			}
		}
	}

	pub fn by_tokens(mut tokens : Vec<String>) -> Result<Cell, ()> {
		// if there is only one token => return it as simple cell
		if tokens.len() == 0 {
			panic!("Cell::by_tokens(): no tokens!");
		} else if tokens.len() == 1 {
			if ! ::is_valid_id(&tokens[0]) {
				panic!("Cell::by_tokens(): invalid id");
			}

			return Ok(Cell::simple(tokens[0].to_string()));
		} else {
			// starts with '('
			if tokens[0] == "(" {
				tokens.remove(0);
			}
			// ends with ')'
			if tokens[tokens.len()-1] == ")" {
				tokens.pop();
			}

			let mut vec : Vec<Cell> = Vec::new();
			let mut parens = 0;
			let mut index = 0;

			while ! tokens.is_empty() {
				let token : String = tokens[0].to_string();
				if ! ::is_valid_id(&token) {
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
					let mut subtokens : Vec<String> = Vec::new();
					let tokens_clone = tokens.clone();
					let (subtokens_slice, tokens_slice) = tokens_clone.split_at(index + 1);
					tokens.clear();
					for x in tokens_slice {
						tokens.push(x.to_string());
					}
					for x in subtokens_slice {
						subtokens.push(x.to_string());
					}
					match Cell::by_tokens(subtokens) {
						Ok(x) => {
							vec.push(x);
						},
						_ => panic!("Cell::by_tokens(): recursive call failed")
					}
					index = 0;
				} else {
					index += 1;
				}
			}
			return Ok(Cell::complex(vec));
		}
	}
}

#[test]
fn test_cell_by_tokens() {
	assert_eq!(Cell::simple("wow".to_string()),
		Cell::by_tokens(vec!["wow".to_string()]).unwrap());
	assert_eq!(Cell::complex(vec![Cell::simple("equals".to_string()), Cell::simple("a".to_string()), Cell::simple("b".to_string())]),
		Cell::by_tokens(vec!["(".to_string(), "equals".to_string(), "a".to_string(), "b".to_string(), ")".to_string()]).unwrap());
}
