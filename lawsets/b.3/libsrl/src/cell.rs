use misc::*;

use std::fmt;
use error::SRLError;

#[derive(PartialEq, Clone)]
pub enum Cell {
	Simple { string : String },
	Complex { cells: Vec<Cell> },
	Scope { id : u32, body : Box<Cell> }, // { ... }
	Var { id : u32 },
	Case { condition : Box<Cell>, conclusion : Box<Cell> }
}

#[derive(PartialEq)]
enum CellType { Simple, Complex, Scope, Var, Case }

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
				string.push_str("[=> ");
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

	pub fn with_subcell(&self, cell : Cell, index : usize) -> Cell {
		if !index_in_len(index, self.count_subcells()) {
			panic!("Cell::with_subcell(): index not possible!");
		}
		match &self {
			&&Cell::Complex { cells : ref cells_out } => {
				let mut c : Vec<Cell> = cells_out.clone();
				c[index] = cell;
				return Cell::Complex { cells : c };
			},
			&&Cell::Scope { id : id_out, .. } => {
				return Cell::Scope { id : id_out, body : Box::new(cell) };
			},
			&&Cell::Case { condition : ref cond_out, conclusion : ref conc_out } => {
				if index == 0 {
					return Cell::Case { condition : Box::new(cell), conclusion : conc_out.clone() };
				} else if index == 1 {
					return Cell::Case { condition : cond_out.clone() , conclusion : Box::new(cell) };
				} else {
					panic!("Cell::with_subcell(): Case out of range");
				}
			},
			_ => panic!("Cell::with_subcell(): unacceptable!")
		}
	}

	pub fn recurse<T>(&self, mut t : T, lambda_expr : fn(&Cell, T) -> T) -> T {
		t = lambda_expr(&self, t);
		for index in 0..self.count_subcells() {
			t = self.get_subcell(index).recurse(t, lambda_expr);
		}
		t
	}

	fn get_type(&self) -> CellType {
		match self {
			&Cell::Scope{..} => CellType::Scope,
			&Cell::Simple{..} => CellType::Simple,
			&Cell::Complex{..} => CellType::Complex,
			&Cell::Case{..} => CellType::Case,
			&Cell::Var{..} => CellType::Var,
		}
	}

	// {0 = {1 (p 0 1) } {2 (p 0 2)}}.
	// the scopes 1 and 2 would `match`, but not be `equal`
	// == equals, when normalized (which you can't do because of vars out of scopes, as seen in example above)
	pub fn matches(&self, cell : &Cell) -> bool {
		self.matches_r(cell, vec![], vec![])
	}

	fn matches_r(&self, cell : &Cell, mut v1 : Vec<usize>, mut v2 : Vec<usize>) -> bool {
		if self.get_type() != cell.get_type() {
			return false;
		}
		if let &Cell::Var { id : id_out } = self {
			let id_out = &(id_out as usize);
			if let &Cell::Var { id : id_out2 } = cell {
				let id_out2  = &(id_out2 as usize);
				if v1.contains(id_out) != v2.contains(id_out2) {
					return false;
				}
				if v1.contains(id_out) {
					v1.iter().position(|x| x == id_out) == v2.iter().position(|x| x == id_out2);
				} else {
					return id_out == id_out2;
				}
			} else { panic!("whoah!") }
		}
		if let &Cell::Scope { id : id_out, .. } = self {
			if let &Cell::Scope { id : id_out2, .. } = cell {
				v1.push(id_out as usize);
				v2.push(id_out2 as usize);
			} else { panic!("whoah!") }
		}
		if self.count_subcells() != cell.count_subcells() {
			return false;
		}
		for i in 0..self.count_subcells() {
			if !self.get_subcell(i).matches(&cell.get_subcell(i)) {
				return false;
			}
		}
		true
		
	}

	pub fn replace_all(&self, pattern : Cell, replacement : Cell) -> Result<Cell, SRLError> {
		let mut cell = self.clone();
		if cell == pattern {
			return Ok(replacement);
		}
		for i in 0..cell.count_subcells() {
			cell = cell.with_subcell(cell.get_subcell(i).replace_all(pattern.clone(), replacement.clone())?, i);
		}
		Ok(cell)
	}
}

#[test]
fn test_get_subcell() {
	assert_eq!(complex(vec![true_cell(), false_cell()]).get_subcell(0), true_cell());
	assert_eq!(complex(vec![true_cell(), false_cell()]).get_subcell(1), false_cell());
}

#[test]
fn test_with_subcell() {
	assert_eq!(complex(vec![true_cell(), true_cell()]).with_subcell(false_cell(), 0), complex(vec![false_cell(), true_cell()]));
	assert_eq!(complex(vec![true_cell(), true_cell()]).with_subcell(false_cell(), 1), complex(vec![true_cell(), false_cell()]));
}

#[test]
fn test_to_string() {
	assert_eq!(&simple_by_str("a").to_string(), "a");
	assert_eq!(&complex(vec![simple_by_str("a"), simple_by_str("b")]).to_string(), "(a b)");
	assert_eq!(&scope(3, simple_by_str("b")).to_string(), "{3 b}");
}
