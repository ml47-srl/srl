pub mod mani;
pub mod create;

use misc::*;
use error::SRLError;
use cell::mani::{simple, var, scope, case, complex};

use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Cell {
	Simple { string : String },
	Complex { cells: Vec<Cell> },
	Scope { id : u32, body : Box<Cell> }, // { ... }
	Var { id : u32 },
	Case { condition : Box<Cell>, conclusion : Box<Cell> }
}

impl fmt::Debug for Cell {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

impl fmt::Display for Cell {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

impl Cell {
	pub fn is_constant(&self) -> bool {
		match &self {
			&&Cell::Simple { string : ref string_out } => return string_out.starts_with('\'') && string_out.ends_with('\''),
			_ => return false
		}
	}

	pub fn is_valid(&self) -> bool {
		match &self {
			&&Cell::Simple { string : ref string_out } => {
				let mut too_many_ticks = string_out.matches('\'').count();
				if self.is_constant() {
					too_many_ticks -= 2;
				}
				return too_many_ticks == 0;
			}
			&&Cell::Complex { cells : ref cells_out } => {
				for cell in cells_out {
					if ! cell.is_valid() {
						return false;
					}
				}
				return true;
			}
			&&Cell::Case { condition : ref cond_out, conclusion : ref conc_out } => {
				return cond_out.is_valid() && conc_out.is_valid();
			}
			_ => return true // TODO Scope / Var
		}
	}

	pub fn to_string(&self) -> String { // (equals a b); a
		return match &self {
			&&Cell::Complex {..} => {
				"(".to_string() + &self.to_unwrapped_string() + ")"
			},
			_ => self.to_unwrapped_string()
		}
	}

	pub fn to_unwrapped_string(&self) -> String { // equals a b | a
		match &self {
			&&Cell::Simple { string : ref string_out } => string_out.to_string(),
			&&Cell::Complex { cells : ref cells_out } => {
				let mut string = String::new();
				string.push_str(&cells_out[0].to_string());
				for cell in cells_out.iter().skip(1) {
					string.push(' ');
					string.push_str(&cell.to_string());
				}
				return string;
			},
			&&Cell::Scope { id : ref id_out, body : ref body_out } => {
				let mut string = String::new();
				string.push('{');
				string.push_str(&id_out.to_string());
				string.push(' ');
				string.push_str(&body_out.to_string());
				string.push('}');
				string
			},
			&&Cell::Var { id : ref id_out } => {
				id_out.to_string()
			},
			&&Cell::Case { condition : ref condition_out, conclusion : ref conclusion_out } => {
				let mut string = String::new();
				string.push('[');
				string.push_str(&condition_out.to_string());
				string.push(' ');
				string.push_str(&conclusion_out.to_string());
				string.push(']');
				string
			}
		}
	}

	pub fn to_rule_string(&self) -> String {
		self.to_unwrapped_string() + "."
	}

	pub fn count_subcells(&self) -> usize {
		return match &self {
			&&Cell::Simple {..} => 0,
			&&Cell::Complex { cells : ref cells_out } => cells_out.len(),
			&&Cell::Scope {..} => 1,
			&&Cell::Var {..} => 0,
			&&Cell::Case {..} => 2
		};
	}

	pub fn get_subcell(&self, index : usize) -> Cell {
		return match &self {
			&&Cell::Simple {..} => panic!("Cell::get_subcell(): Simple: no subcells"),
			&&Cell::Complex { cells : ref cells_out } => {
				if ! index_in_len(index, cells_out.len()) {
					panic!("Cell::get_subcell(): Complex: index out of range")
				}
				cells_out[index].clone()
			}
			&&Cell::Scope { body : ref body_out, .. } => {
				if index == 0 {
					body_out.as_ref().clone()
				} else {
					panic!("Cell::get_subcell(): Scope: index out of range")
				}
			},
			&&Cell::Var {..} => panic!("Cell::get_subcell(): Var: no subcells"),
			&&Cell::Case { condition : ref cond_out, conclusion : ref conc_out } => {
				if index == 0 {
					return cond_out.as_ref().clone();
				}
				if index == 1 {
					return conc_out.as_ref().clone();
				} 
				panic!("Cell::get_subcell(): Case: index out of range");
			}
		};
	}

	fn get_scope_ids_r(&self, mut scope_ids : Vec<u32>) -> Result<Vec<u32>, SRLError> {
		if let &&Cell::Scope { id : id_out, .. } = &self {
			if scope_ids.contains(&id_out) {
				return Err(SRLError("get_scope_ids_r".to_string(), format!("id {} occured multiple times", id_out)));
			}
			scope_ids.push(id_out);
		}

		for index in 0..(self.count_subcells() as i32)-1 {
			match self.get_subcell(index as usize).get_scope_ids_r(scope_ids) {
				Ok(scope_ids_new) => { scope_ids = scope_ids_new; },
				Err(srl_error) => return Err(srl_error)
			}
		}

		Ok(scope_ids)
	}

	fn get_scope_ids(&self) -> Result<Vec<u32>, SRLError> {
		return self.get_scope_ids_r(vec![]);
	}

	fn get_replaced_scope_ids(&self, scope_ids : &Vec<u32>) -> Result<Cell, SRLError> {
		match &self {
			&&Cell::Scope { id : id_out, body : ref body_out } => {
				match (*body_out).get_replaced_scope_ids(&scope_ids) {
					Ok(new_body) => {
						match get_new_id(id_out, &scope_ids) {
							Ok(new_id) => return Ok(scope(new_id, new_body)),
							Err(srl_error) => return Err(srl_error)
						}
					}
					Err(srl_error) => return Err(srl_error)
				}
			},
			&&Cell::Var { id : id_out } => {
				match get_new_id(id_out, &scope_ids) {
					Ok(new_id) => return Ok(var(new_id)),
					Err(srl_error) => return Err(srl_error)
				}
			},
			&&Cell::Complex { cells : ref cells_out } => {
				let mut new_cells = Vec::new();
				for cell in cells_out {
					match cell.get_replaced_scope_ids(&scope_ids) {
						Ok(cell) => {
							new_cells.push(cell);
						},
						Err(srl_error) => return Err(srl_error)
					}
				}
				return Ok(complex(new_cells));
			}
			&&Cell::Simple { string : ref string_out } => return Ok(simple(string_out.to_string())),
			&&Cell::Case {condition : ref condition_out, conclusion : ref conclusion_out} => {
				match (*condition_out).get_replaced_scope_ids(&scope_ids) {
					Ok(new_condition) => {
						match (*conclusion_out).get_replaced_scope_ids(&scope_ids) {
							Ok(new_conclusion) => {
								return Ok(case(new_condition, new_conclusion));
							}
							Err(srl_error) => return Err(srl_error)
						}
					}
					Err(srl_error) => return Err(srl_error)
				}
			}
		}
	}

	// creates new cell with normalized scopes
	// -- errors on var out of scope/multiple scopes with same id
	pub fn get_normalized(&self) -> Result<Cell, SRLError> {
		match self.get_scope_ids() {
			Ok(scope_ids) => {
				return self.get_replaced_scope_ids(&scope_ids);
			}
			Err(srl_error) => return Err(srl_error)
		}
	}
}

fn get_new_id(old_id : u32, scope_ids : &Vec<u32>) -> Result<u32, SRLError> {
	for index in 0..scope_ids.len()-1 {
		if old_id == scope_ids[index] {
			return Ok(index as u32);
		}
	}
	return Err(SRLError("get_new_id".to_string(), format!("id '{}' is not in scope_ids", old_id)));
}

#[test]
fn test_to_string() {
	use cell::mani::*;
	assert_eq!(&simple_by_str("a").to_string(), "a");
	assert_eq!(&complex(vec![simple_by_str("a"), simple_by_str("b")]).to_string(), "(a b)");
	assert_eq!(&scope(3, simple_by_str("b")).to_string(), "{3 b}");
}
