pub mod reason;

use cell::Cell;
use std::fs::File;
use std::io::Read;
use misc::*;
use gen::*;
use error::SRLError;

pub struct Database {
	rules : Vec<Cell>,
	src_rules_count : usize
}

impl Database {
	pub fn by_string(string : &str) -> Result<Database, SRLError> {
		use parse::*;

		let rule_strings = split_rules(string.trim().to_string());
		let mut rules : Vec<Cell> = vec![scope(0, complex(vec![simple_by_str("="), var(0), var(0)]))];
		for rule_string in rule_strings {
			rules.push(Cell::by_string(&rule_string)?.get_normalized()?);
		}
		let len = rules.len();
		Ok(Database { rules : rules, src_rules_count : len })
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

	pub fn get_rules(&self) -> Vec<Cell> {
		self.rules.clone()
	}

	pub fn get_rule(&self, index : usize) -> Cell {
		if ! index_in_len(index, self.rules.len()) {
			panic!(format!("Database::get_rule({}): index out of range", index));
		}
		self.rules[index].clone()
	}

	pub fn delete_rule(&mut self, index : usize) -> Result<(), SRLError> {
		if index_in_len(index, self.src_rules_count) {
			return Err(SRLError("Database::delete_rule".to_string(), "This rule is write protected".to_string()))
		}
		if index_in_len(index, self.count_rules()) {
			self.rules.remove(index);
			return Ok(());
		}
		return Err(SRLError("Database::delete_rule".to_string(), "out of range".to_string()))
	}
}
