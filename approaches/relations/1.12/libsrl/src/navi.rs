use cell::Cell;
use cell::mani::*;

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
					},
					Cell::Scope { id : id_out, body : body_out } => {
						if index == 0 {
							cell = *id_out;
						} else if index == 1 {
							cell = *body_out;
						} else {
							panic!("CellID::get_cell(): index different than 0 or 1 for scope cell");
						}
					},
					Cell::Var { id : id_out } => {
						if index == 0 {
							cell = *id_out;
						} else {
							panic!("CellID::get_cell(): index different than 0 for var cell");
						}
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
					cell = complex(cells_out);
				}
				Cell::Simple {..} => panic!("CellID::replace_by: failure 2"),
				Cell::Scope { id : mut id_out, body : mut body_out } => {
					if last_index == 0 {
						id_out = Box::new(cell);
					} else if last_index == 1 {
						body_out = Box::new(cell);
					} else {
						panic!("CellID::replace_by(): index different than 0 or 1 for scope cell");
					}
					cell = scope(*id_out, *body_out);
				},
				Cell::Var { id : mut id_out } => {
					if last_index == 0 {
						id_out = Box::new(cell);
					} else {
						panic!("CellID::replace_by(): index different than 0 for var cell");
					}

					cell = var(*id_out);
				}
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
				Cell::Simple {..} => return false,
				Cell::Complex { cells : cells_out } => {
					cell = cells_out[index].clone();
				},
				Cell::Scope { id : id_out, body : body_out } => {
					if index == 0 {
						cell = *id_out;
					} else if index == 1 {
						cell = *body_out;
					} else {
						panic!("CellID::is_valid(): index different than 0 or 1 for scope cell");
					}
				},
				Cell::Var { id : id_out } => {
					if index == 0 {
						cell = *id_out;
					} else {
						panic!("CellID::is_valid(): index different than 0 for var cell");
					}
				}
			}
		}
		true
	}
}
