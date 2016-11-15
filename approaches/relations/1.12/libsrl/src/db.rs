use parse::*;
use cell::Cell;
use std::fs::File;
use std::io::Read;
use interface::apply::ApplyInterface;
use interface::paradox::ParadoxInterface;
use interface::equals_evi::EqualsEvidenceInterface;
use interface::differ_evi::DifferEvidenceInterface;

pub struct Database {
	rules : Vec<Cell>
}

impl Database {
	pub fn by_string(string : &str) -> Result<Database, String> {
		match find_invalid_char(&string) {
			Some(x) => panic!("Inaccepted characters in string; char_no = {}", x),
			None => {}
		}
		let string : String = fix_whitespaces(string);
		let rule_strings = split_rules(string);
		let mut rules : Vec<Cell> = Vec::new();
		for rule_string in rule_strings {
			let tokens : Vec<String> = split_tokens(rule_string);
			match Cell::by_tokens(tokens) {
				Ok(x) => {
					rules.push(x);
				},
				Err(_) => panic!("Database::by_string(): Cell::by_tokens() failed")
			}
		}
		for rule in &rules {
			if ! rule.is_valid() {
				return Err(format!("rule '{}' is malformed", rule.to_rule_string()));
			}
		}
		Ok(Database { rules : rules })
	}

	pub fn by_filename(filename : &str) -> Result<Database, String> {
		let mut file : File = match File::open(filename) {
			Ok(file) => file,
			Err(_) => return Err(format!("Cannot open file: '{}'", filename)),
		};
		let mut filecontent = String::new();
		match file.read_to_string(&mut filecontent) {
			Ok(_) => (),
			Err(_) => return Err(format!("failed to read from file: '{}'", filename))
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

	pub fn count_rules(&self) -> usize
	{
		self.rules.len()
	}

	pub fn get_rule(&self, index : usize) -> Cell
	{
		self.rules[index].clone()
	}
}
