use parse::*;
use cell::Cell;
use std::fs::File;
use std::io::Read;
use interface::apply_interface::ApplyInterface;
use interface::paradox_interface::ParadoxInterface;
use interface::equals_evi_interface::EqualsEvidenceInterface;
use interface::difference_evi_interface::DifferenceEvidenceInterface;

pub struct Database {
	rules : Vec<Cell>
}

impl Database {
	pub fn by_string(string : &str) -> Database {
		match find_invalid_char(&string) {
			Some(x) => panic!("Inaccepted characters in string; char_no = {}", x),
			None => {}
		}
		let string : String = fix_whitespaces(string);
		let rule_strings = split_rules(string);
		let mut rules : Vec<Cell> = Vec::new();
		for rule_string in rule_strings {
			let rule_string : String = one_layer_parens(&rule_string);
			let tokens : Vec<String> = split_tokens(rule_string);
			match Cell::by_tokens(tokens) {
				Ok(x) => {
					rules.push(x);
				},
				Err(_) => panic!("Database::by_string(): Cell::by_tokens() failed")
			}
		}
		Database { rules : rules }
	}

	pub fn by_filename(filename : &str) -> Database {
		let mut file : File = match File::open(filename) {
			Ok(file) => file,
			Err(_) => panic!("failed to open file")
		};
		let mut filecontent = String::new();
		match file.read_to_string(&mut filecontent) {
			Ok(_) => (),
			Err(_) => panic!("failed to read from file")
		}
		Database::by_string(&filecontent)
	}

	pub fn apply_interface(&mut self) -> ApplyInterface {
		ApplyInterface(&mut self.rules)
	}

	pub fn paradox_interface(&self) -> ParadoxInterface {
		ParadoxInterface(&self.rules)
	}

	pub fn equals_evi_interface(&self) -> EqualsEvidenceInterface {
		EqualsEvidenceInterface(&self.rules)
	}

	pub fn difference_evi_interface(&self) -> DifferenceEvidenceInterface {
		DifferenceEvidenceInterface(&self.rules)
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
