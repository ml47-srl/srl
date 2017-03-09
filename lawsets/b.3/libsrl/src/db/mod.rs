pub mod reason;

use parse::*;
use parse::assemble::*;
use parse::tokenize::*;
use cell::Cell;
use std::fs::File;
use std::io::Read;
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
			let tokens = tokenize(rule_string)?;
			if ! check_paren_correctness(tokens.clone()) {
				return Err(SRLError("Database::by_string()".to_string(), "Parens are not correct".to_string()));
			}
			let cell = assemble(tokens)?;
			let normalized = cell.get_normalized()?;
			rules.push(normalized);
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
		if let Err(_) = file.read_to_string(&mut filecontent) {
			return Err(SRLError("Database::by_filename".to_string(), format!("failed to read from file: '{}'", filename)));
		}
		Database::by_string(&filecontent)
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
