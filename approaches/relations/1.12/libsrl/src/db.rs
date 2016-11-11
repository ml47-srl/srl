use parse::*;
use cell::Cell;
use navi::RuleID;
use navi::CellID;
use evi::EqualsEvidence;
use evi::DifferenceEvidence;
use std::fs::File;
use std::io::Read;

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

	pub fn apply_equals(&mut self, evi : EqualsEvidence) -> Result<Cell, String> {
		let result = Cell::complex(vec![Cell::simple_by_str("equals"), evi.0, evi.1]);
		self.rules.push(result.clone());
		Ok(result)
	}

	pub fn apply_equals_change(&mut self, equals_evidence : &EqualsEvidence, target_cell_id : &CellID) -> Result<Cell, String> { // returns and adds rule
		if target_cell_id.is_valid(&self.rules) {
			let cell = target_cell_id.get_cell(&self.rules);
			if equals_evidence.0 == cell {
				let c = target_cell_id.replace_by(&self.rules, equals_evidence.1.clone());
				self.rules.push(c.clone());
				return Ok(c);
			} else if equals_evidence.1 == cell {
				let c = target_cell_id.replace_by(&self.rules, equals_evidence.0.clone());
				self.rules.push(c.clone());
				return Ok(c);
			} else {
				return Err("wrong member of equals_evidence".to_string());
			}
		} else {
			Err("target_cell_id is invalid".to_string())
		}
	}

	fn on_paradox(&self) {
		println!("The Database is paradox. Something has gone wrong here..");
		panic!("PARADOX!");
	}

	pub fn evidence_paradox_equal_and_differ(&self, equals_evidence : &EqualsEvidence, differ_evidence : &DifferenceEvidence) -> Result<(), String> {
		if (equals_evidence.0 == differ_evidence.0 && equals_evidence.1 == differ_evidence.1) || (equals_evidence.0 == differ_evidence.1 && equals_evidence.1 == differ_evidence.0) {
			self.on_paradox();
			Ok(())
		} else {
			Err("Database::evidence_paradox_equal_and_differ(): wrong cells".to_string())
		}
	}

	pub fn evidence_equals_by_rule(&self, rule_id : &RuleID) -> Result<EqualsEvidence, String> {
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

	pub fn evidence_equals_same_cell(&self, cell : Cell) -> Result<EqualsEvidence, String> {
		if cell.is_valid() {
			Ok(EqualsEvidence(cell.clone(), cell))
		} else {
			Err("invalid cell".to_string())
		}
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
