pub mod mani;
pub mod create;

use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Cell {
	Simple { string : String },
	Complex { cells: Vec<Cell> },
	Scope { id : Box<Cell>, body : Box<Cell> }, // { ... }
	Var { id : Box<Cell> } // [ ... ]
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
			_ => return true // TODO Scope / Var
		}
	}

	pub fn to_string(&self) -> String { // (equals a b); a
		return match &self {
			&&Cell::Simple { string : _ } => {
				self.to_unwrapped_string()
			},
			&&Cell::Complex { cells : _ } => {
				"(".to_string() + &self.to_unwrapped_string() + ")"
			},
			_ => "<tmp>".to_string() // TODO
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
			_ => "<tmp>".to_string() // TODO
		}
	}

	pub fn to_rule_string(&self) -> String {
		self.to_unwrapped_string() + "."
	}

	pub fn count_subcells(&self) -> usize {
		return match &self {
			&&Cell::Simple {..} => 0,
			&&Cell::Complex { cells : ref cells_out } => cells_out.len(),
			&&Cell::Scope {..} => 2,
			&&Cell::Var {..} => 1
		};
	}

	pub fn get_subcell(&self, index : usize) -> Cell {
		return match &self {
			&&Cell::Complex { cells : ref cells_out } => {
				if index > cells_out.len()-1 {
					panic!("Cell::get_subcell(): Complex: index out of range")
				}
				cells_out[index].clone()
			}
			&&Cell::Scope { id : ref id_out, body : ref body_out } => {
				if index == 0 {
					id_out.as_ref().clone()
				} else if index == 1 {
					body_out.as_ref().clone()
				} else {
					panic!("Cell::get_subcell(): Scope: index out of range")
				}
			},
			&&Cell::Var { id: ref id_out } => {
				if index != 0 {
					panic!("Cell::get_subcell(): Var: index out of range");
				}
				id_out.as_ref().clone()
			},
			&&Cell::Simple { .. } => panic!("Cell::get_subcell(): Simple: index out of range")
		};
	}
}
