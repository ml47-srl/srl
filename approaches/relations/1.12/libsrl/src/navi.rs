use cell::Cell;

#[derive(Clone)]
pub struct RuleID(usize);

#[derive(Clone)]
pub struct CellID {
	rule_id : RuleID,
	indices : Vec<usize>
}

impl RuleID {
	pub fn get_cell(&self, rules : &Vec<Cell>) -> Cell {
		if self.is_valid(rules) {
			return rules[self.0].clone()
		} else {
			panic!("RuleID::get_rule_cell: id='{}' out of range", self.0);
		}
	}

	pub fn is_valid(&self, rules : &Vec<Cell>) -> bool {
		rules.len()-1 >= self.0
	}
}

impl CellID {
	pub fn get_cell(&self, rules : &Vec<Cell>) -> Cell {
		if self.rule_id.is_valid(rules) {
			let mut cell : Cell = self.rule_id.get_cell(rules); // obtain copy
			for index in self.indices.clone() {
				match cell {
					Cell::Simple { string : string_out } => panic!("expected ComplexCell, got SimpleCell: '{}'", string_out),
					Cell::Complex { cells : cells_out } => {
						cell = cells_out[index].clone()
					}
				}
			}
			return cell;
		} else {
			panic!("CellID::get_cell(): rule_id is invalid");
		}
	}
	
	pub fn replace_by(&self, rules : &Vec<Cell>, mut cell : Cell) -> Cell {
		if ! self.rule_id.is_valid(rules) {
			panic!("CellID::replace_by(): rule_id is invalid");
		}

		let mut indices = self.indices.clone();
		let mut last_index;

		while indices.len() > 0 {
			match indices.pop() {
				Some(x) => last_index = x,
				None => panic!("CellID::replace_by(): failure 1")
			}
			let cell_id = CellID { rule_id : self.rule_id.clone(), indices : indices.clone() };
			match cell_id.get_cell(rules) {
				Cell::Complex { cells : mut cells_out } => {
					cells_out[last_index] = cell;
					cell = Cell::complex(cells_out);
				}
				Cell::Simple { string : _ } => panic!("CellID::replace_by: failure 2")
			}
		}
		return cell;
	}

	pub fn is_valid(&self, rules : &Vec<Cell>) -> bool {
		if ! self.rule_id.is_valid(rules) {
			return false;
		}

		let mut cell : Cell = self.rule_id.get_cell(rules); // obtain copy
		for index in self.indices.clone() {
			match cell {
				Cell::Simple { string : _ } => return false,
				Cell::Complex { cells : cells_out } => {
					cell = cells_out[index].clone();
				}
			}
		}
		true
	}
}
