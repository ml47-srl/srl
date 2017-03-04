use cell::Cell;
use misc::*;
use error::SRLError;

#[derive(Clone)]
pub struct RuleID(usize);

#[derive(Clone)]
pub struct CellID {
	rule_id : RuleID,
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
	pub fn get_cell(&self, rules : &Vec<Cell>) -> Result<Cell, SRLError> {
		if self.rule_id.is_valid(rules) {
			let mut cell = match self.rule_id.get_cell(rules) { // obtain copy
				Ok(x) => x,
				Err(srl_error) => return Err(srl_error)
			};
			for index in self.indices.clone() {
				cell = cell.get_subcell(index)
			}
			return Ok(cell);
		} else {
			return Err(SRLError("CellID.get_cell".to_string(), "invalid cell".to_string()));
		}
	}

	pub fn replace_by(&self, rules : &Vec<Cell>, mut cell : Cell) -> Result<Cell, SRLError> {
		let mut indices = self.indices.clone();
		let mut last_index;

		while indices.len() > 0 {
			match indices.pop() {
				Some(x) => last_index = x,
				None => panic!("CellID.replace_by: failure 1 - should not happen")
			}
			let cell_id = CellID { rule_id : self.rule_id.clone(), indices : indices.clone() };
			match cell_id.get_cell(rules) {
				Ok(Cell::Complex { cells : mut cells_out }) => {
					cells_out[last_index] = cell;
					cell = complex(cells_out);
				}
				Ok(Cell::Simple {..}) => return Err(SRLError("CellID.replace_by".to_string(), "failure simple-cell".to_string())),
				Ok(Cell::Scope { id: id_out, body : mut body_out }) => {
					if last_index == 0 {
						body_out = Box::new(cell);
					} else {
						return Err(SRLError("CellID.replace_by".to_string(), "index different than 0 for scope cell".to_string()));
					}
					cell = scope(id_out, *body_out);
				},
				Ok(Cell::Var {..}) => return Err(SRLError("CellID.replace_by".to_string(), "failure var-cell".to_string())),
				Ok(Cell::Case { condition : mut cond_out, conclusion : mut conc_out }) => {
					if last_index == 0 {
						cond_out = Box::new(cell);
					} else if last_index == 1 {
						conc_out = Box::new(cell);
					} else {
						return Err(SRLError("CellID.replace_by".to_string(), "index different than 0 or 1 for case cell".to_string()));
					}
					cell = case(*cond_out, *conc_out);
				},
				Err(srl_error) => return Err(srl_error)
			}
		}
		return Ok(cell);
	}

	pub fn get_parent(&self) -> Result<CellID, SRLError> {
		let mut vec = self.indices.clone();
		match vec.pop() {
			Some(_) => return Ok(CellID { rule_id : self.rule_id.clone(), indices : vec }),
			None => return Err(SRLError("CellID.get_parent".to_string(), "no parent".to_string()))
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
}

#[test]
fn test_cell_id() {
	let mut rules : Vec<Cell> = Vec::new();
	rules.push(simple_by_str("truth"));
	rules.push(complex(vec![simple_by_str("truth"), simple_by_str("wot")]));

	assert!(CellID { rule_id : RuleID(0), indices : Vec::new() }.is_valid(&rules));
	assert!(CellID { rule_id : RuleID(1), indices : Vec::new() }.is_valid(&rules));
	assert!(! CellID { rule_id : RuleID(2), indices : Vec::new() }.is_valid(&rules));

	assert_eq!(
		CellID { rule_id : RuleID(0), indices : Vec::new() }.get_cell(&rules).unwrap(),
		simple_by_str("truth")
	);

	assert_eq!(
		CellID { rule_id : RuleID(1), indices : vec![0] }.get_cell(&rules).unwrap(),
		simple_by_str("truth")
	);

	assert_eq!(
		CellID { rule_id : RuleID(1), indices : vec![1] }.get_cell(&rules).unwrap(),
		simple_by_str("wot")
	);
}

#[test]
fn test_cell_id_replace_by() {
	let mut rules : Vec<Cell> = Vec::new();
	rules.push(simple_by_str("truth"));
	rules.push(complex(vec![simple_by_str("truth"), simple_by_str("wot")]));

	assert_eq!(
		CellID { rule_id : RuleID(1), indices : vec![1] }.replace_by(&rules, simple_by_str("wow")).unwrap(),
		complex(vec![simple_by_str("truth"), simple_by_str("wow")])
	);
}
