use cell::Cell;
use misc::*;
use error::SRLError;

#[derive(Clone, PartialEq)]
pub struct RuleID(usize);

#[derive(Clone, PartialEq)]
pub struct CellID {
	rule_id : RuleID,
	indices : Vec<usize>
}

#[derive(Clone, PartialEq)]
pub struct CellPath {
	root_cell : Cell,
	indices : Vec<usize>
}

impl RuleID {
	pub fn get_cell(&self, rules : &Vec<Cell>) -> Result<Cell, SRLError> {
		if self.is_valid(rules) {
			return Ok(rules[self.0].clone());
		} else {
			return Err(SRLError("RuleID.get_cell".to_string(), "invalid RuleID".to_string()));
		}
	}

	fn is_valid(&self, rules : &Vec<Cell>) -> bool {
		index_in_len(self.0, rules.len())
	}
}

impl CellID {
	pub fn create(rule_index : usize, indices : Vec<usize>) -> CellID {
		CellID { rule_id : RuleID(rule_index), indices : indices }
	}

	pub fn get_path(&self, rules : &Vec<Cell>) -> Result<CellPath, SRLError> {
		if index_in_len(self.rule_id.0, rules.len()) {
			Ok(CellPath { root_cell : rules[self.rule_id.0].clone(), indices : self.indices.clone() })
		} else {
			Err(SRLError("CellID::to_path".to_string(), "index of rule_id out of range".to_string()))
		}
	}

	fn is_valid(&self, rules : &Vec<Cell>) -> bool {
		if ! self.rule_id.is_valid(rules) {
			return false;
		}

		let mut cell = match self.rule_id.get_cell(rules) { // obtain copy
			Ok(x) => x,
			Err(_) => return false
		};
			
		for index in self.indices.clone() {
			if ! index_in_len(index, cell.count_subcells()) {
				return false;
			}
			cell = cell.get_subcell(index);
		}
		return true;
	}

	pub fn get_rule_id(&self) -> RuleID { self.rule_id.clone() }
	pub fn get_indices(&self) -> Vec<usize> { self.indices.clone() }

}

impl CellPath {
	pub fn create(root_cell : Cell, indices : Vec<usize>) -> CellPath {
		CellPath { root_cell : root_cell, indices : indices }
	}

	pub fn get_cell(&self) -> Result<Cell, SRLError> {
		let mut cell = self.root_cell.clone();
		for index in self.indices.clone() {
			cell = cell.get_subcell(index);
		}
		Ok(cell)
	}

	pub fn is_complete_bool(&self) -> bool {
		let my_cell = match self.get_cell() {
			Ok(x) => x,
			Err(_) => return false
		};
		if let Ok(_) = my_cell.get_equals_cell_arguments() { return true; } // it's an equals cell
		if let Cell::Scope{..} = my_cell { return true; } // it's a scope
		if my_cell == true_cell() { return true; }
		if my_cell == false_cell() { return true; }
		false
	}

	pub fn is_bool(&self) -> bool {
		if self.is_complete_bool() {
			return true;
		}

		// positionals
		if !self.indices.is_empty() {
			match self.get_parent().get_cell() {
				Ok(Cell::Scope{..}) | Ok(Cell::Case{..}) => return true,
				_ => {}
			}
		}
		false
	}

	pub fn get_parent(&self) -> CellPath {
		let mut vec = self.indices.clone();
		match vec.pop() {
			Some(_) => return CellPath { root_cell : self.root_cell.clone(), indices : vec },
			None => panic!("CellID.get_parent: no parent")
		}
	}

	pub fn get_child(&self, index : usize) -> CellPath {
		let mut vec = self.indices.clone();
		vec.push(index);
		CellPath { root_cell : self.root_cell.clone(), indices : vec }
	}

	pub fn replace_by(&self, mut cell : Cell) -> Result<Cell, SRLError> {
		let mut indices = self.indices.clone();
		let mut last_index;

		while indices.len() > 0 {
			last_index = match indices.pop() {
				Some(x) => x,
				None => panic!("CellPath.replace_by: failure 1 - should not happen")
			};
			let cell_path = CellPath { root_cell : self.root_cell.clone(), indices : indices.clone() };
			cell = match cell_path.get_cell() {
				Ok(x) => x.with_subcell(cell, last_index),
				Err(srl_error) => return Err(srl_error)
			};
		}
		Ok(cell)
	}

	pub fn get_root_cell(&self) -> Cell { self.root_cell.clone() }
	pub fn get_indices(&self) -> Vec<usize> { self.indices.clone() }
}

#[test]
fn test_cell_id_and_cell_path() {
	let mut rules : Vec<Cell> = Vec::new();
	rules.push(simple_by_str("truth"));
	rules.push(complex(vec![simple_by_str("truth"), simple_by_str("wot")]));

	assert!(CellID { rule_id : RuleID(0), indices : Vec::new() }.is_valid(&rules));
	assert!(CellID { rule_id : RuleID(1), indices : Vec::new() }.is_valid(&rules));
	assert!(! CellID { rule_id : RuleID(2), indices : Vec::new() }.is_valid(&rules));

	assert_eq!(
		CellPath { root_cell : rules[0].clone(), indices : Vec::new() }.get_cell().unwrap(),
		simple_by_str("truth")
	);

	assert_eq!(
		CellPath { root_cell : rules[1].clone(), indices : vec![0] }.get_cell().unwrap(),
		simple_by_str("truth")
	);

	assert_eq!(
		CellPath { root_cell : rules[1].clone(), indices : vec![1] }.get_cell().unwrap(),
		simple_by_str("wot")
	);
}

#[test]
fn test_cell_path_replace_by() {
	let mut rules : Vec<Cell> = Vec::new();
	rules.push(simple_by_str("truth"));
	rules.push(complex(vec![simple_by_str("truth"), simple_by_str("wot")]));

	assert_eq!(
		CellPath { root_cell : rules[1].clone(), indices : vec![1] }.replace_by(simple_by_str("wow")).unwrap(),
		complex(vec![simple_by_str("truth"), simple_by_str("wow")])
	);
}
