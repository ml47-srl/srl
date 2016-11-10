mod parse;
use parse::*;

mod cell;
use cell::Cell;

mod navi;
use navi::RuleID;
use navi::CellID;

mod evi;
use evi::EqualsEvidence;
use evi::DifferenceEvidence;
use evi::ParadoxEvidence;

use std::fs::File;
use std::io::Read;

pub struct Database {
	#[allow(dead_code)]
	rules : Vec<Cell>
}

impl Database {
	#[allow(dead_code)]
	fn by_string(string : &str) -> Database {
		match find_invalid_char(&string) {
			Some(x) => panic!("Inaccepted characters in string; char_no = {}", x),
			None => {}
		}
		let string : String = fix_whitespaces(string);
		let rule_strings = split_rules(string);
		let mut rules : Vec<Cell> = Vec::new();
		for rule_string in rule_strings {
			let mut tokens : Vec<String> = split_tokens(rule_string);

			// adding implicit parens
			if tokens.len() > 1 && tokens[0] != "(" {
				tokens.insert(0, "(".to_string());
				tokens.push(")".to_string());
			}

			match Cell::by_tokens(tokens) {
				Ok(x) => {
					rules.push(x);
				},
				Err(_) => {
					panic!("Database::by_string(): Cell::by_tokens() failed");
				}
			}
		}
		Database { rules : rules }
	}

	#[allow(dead_code)]
	fn by_filename(filename : &str) -> Database {
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

	fn apply_equals_change(&mut self, equals_evidence : &EqualsEvidence, target_cell_id : &CellID) -> Option<String> {
		// TODO
		None
	}

	fn apply_paradox(&mut self, paradox_evidence : &ParadoxEvidence) -> Option<String> {
		println!("The Database is paradox. Something has gone wrong here..");
		None
	}

	fn evidence_equals(&self, rule_id : &RuleID) -> Result<EqualsEvidence, String> {
		if ! rule_id.is_valid(&self.rules) {
			return Err("rule_id is invalid".to_string());
		}
		let cell = rule_id.get_cell(&self.rules);
		match cell {
			Cell::SimpleCell { string : _ } => return Err("rule_id points to simple cell".to_string()),
			Cell::ComplexCell { cells : cells_out } => {
				if cells_out.len() != 3 {
					return Err(format!("rule_id points to cell with {} arguments", cells_out.len()));
				}
				if cells_out[0].to_string() == "equals" {
					return Ok(EqualsEvidence(cells_out[1].clone(), cells_out[2].clone()));
				} else {
					return Err(format!("rule_id points to cell which starts with '{}'", cells_out[0]));
				}
			}
		}
	}
}
