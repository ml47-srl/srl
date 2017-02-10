use parse::*;
use parse::assemble::*;
use parse::tokenize::*;
use cell::Cell;
use std::fs::File;
use std::io::Read;
use interface::apply::ApplyInterface;
use interface::paradox::ParadoxInterface;
use interface::equals_evi::EqualsEvidenceInterface;
use interface::differ_evi::DifferEvidenceInterface;
use interface::scope::ScopeInterface;
use misc::*;
use error::SRLError;

pub struct Database {
	rules : Vec<Cell>
}

impl Database {
	pub fn by_string(string : &str) -> Result<Database, SRLError> {
		match find_invalid_char(&string) {
			Some(x) => return Err(SRLError("Database::by_string".to_string(), format!("invalid char '{}'", string.chars().nth(x as usize).unwrap()))),
			None => {}
		}
		let string : String = fix_whitespaces(string);
		let rule_strings = split_rules(string);

		// core rule:
		let mut rules : Vec<Cell> = vec![scope(0, complex(vec![simple_by_str("="), var(0), var(0)]))];

		for rule_string in rule_strings {
			match tokenize(rule_string) {
				Ok(tokens) => {
					if ! check_paren_correctness(tokens.clone()) {
						return Err(SRLError("Database::by_string()".to_string(), "Parens are not correct".to_string()));
					}
					match assemble(tokens) {
						Ok(cell) => {
							match cell.get_normalized() {
								Ok(x) => {
									rules.push(x);
								}
								Err(srl_error) => return Err(srl_error)
							}
						},
						Err(srl_error) => return Err(srl_error)
					}
				},
				Err(srl_error) => return Err(srl_error)
			}
		}
		for rule in &rules {
			if ! rule.is_valid() {
				return Err(SRLError("Database::by_string()".to_string(), format!("rule '{}' is malformed", rule.to_rule_string())));
			}
		}
		Ok(Database { rules : rules })
	}

	pub fn by_filename(filename : &str) -> Result<Database, SRLError> {
		let mut file : File = match File::open(filename) {
			Ok(file) => file,
			Err(_) => return Err(SRLError("Database::by_filename".to_string(), format!("Cannot open file: '{}'", filename))),
		};
		let mut filecontent = String::new();
		match file.read_to_string(&mut filecontent) {
			Ok(_) => (),
			Err(_) => return Err(SRLError("Database::by_filename".to_string(), format!("failed to read from file: '{}'", filename)))
		}
		Database::by_string(&filecontent)
	}

	pub fn apply_i(&mut self) -> ApplyInterface {
		ApplyInterface::new(&mut self.rules)
	}

	pub fn paradox_i(&self) -> ParadoxInterface {
		ParadoxInterface::new(&self.rules)
	}

	pub fn equals_evi_i(&self) -> EqualsEvidenceInterface {
		EqualsEvidenceInterface::new(&self.rules)
	}

	pub fn differ_evi_i(&self) -> DifferEvidenceInterface {
		DifferEvidenceInterface::new(&self.rules)
	}

	pub fn scope_i(&mut self) -> ScopeInterface {
		ScopeInterface::new(&mut self.rules)
	}

	pub fn count_rules(&self) -> usize {
		self.rules.len()
	}

	pub fn get_rule(&self, index : usize) -> Cell {
		if ! index_in_len(index, self.rules.len()) {
			panic!(format!("Database::get_rule({}): index out of range", index));
		}
		self.rules[index].clone()
	}
}

// checks for problems like unclosed parens, and ({)}-like things
fn check_paren_correctness(mut tokens : Vec<String>) -> bool {
	let mut index = 0;
	while index_in_len(index, tokens.len()) {
		match "{}[]()".to_string().find(&tokens[index]) {
			Some(_) => {
				index += 1;
			},
			None => {
				tokens.remove(index);
			}
		}
	}

	let mut string = tokens.join("");
	loop {
		match string.find("{}") {
			Some(i) => {
				string.remove(i);
				string.remove(i);
				continue;
			},
			None => {}
		};
		match string.find("()") {
			Some(i) => {
				string.remove(i);
				string.remove(i);
				continue;
			},
			None => {}
		};
		match string.find("[]") {
			Some(i) => {
				string.remove(i);
				string.remove(i);
				continue;
			},
			None => {}
		};
		break;
	}

	return string.is_empty();
}

#[test]
fn test_check_paren_correctness() {
	assert_eq!(check_paren_correctness(vec!["{".to_string(), "(".to_string(), "}".to_string(), ")".to_string()]), false);
	assert_eq!(check_paren_correctness(vec!["{".to_string(), "(".to_string(), ")".to_string(), "}".to_string()]), true);
	assert_eq!(check_paren_correctness(vec!["{".to_string(), "testy".to_string(), "(".to_string(), ")".to_string(), "}".to_string()]), true);
	assert_eq!(check_paren_correctness(vec!["{".to_string(), "testy".to_string(), "(".to_string(), ")".to_string(), "}".to_string()]), true);
}
