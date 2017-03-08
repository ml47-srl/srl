mod wrapper;

use super::Database;
use cell::Cell;
use error::SRLError;
use misc::*;
use navi::CellID;
use navi::CellPath;
use secure::SecureCell;

fn find_highest(cell : &Cell, i : i32) -> i32 {
	if let &Cell::Scope { id : x, ..} = cell {
		if x as i32 > i { x as i32 }
		else { i }
	} else {
		i
	}
}

impl Database {
	fn add_rule(&mut self, rule : Cell) -> Result<Cell, SRLError> {
		match rule.get_normalized() {
			Ok(x) => {
				self.rules.push(x.clone());
				return Ok(x);
			}
			Err(srl_error) => return Err(srl_error)
		}
	}

	// src_id = "The cell that has to be replaced" | `{0 (<p> 0)}.`
	// evidence_id = "the equals cell"		  | `{0 <(= p q)>}`
	pub fn equals_law(&mut self, src_id : CellID, evidence_id : CellID) -> Result<Cell, SRLError> {
		let src_path = match src_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let evidence_path = match evidence_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let wrapper = match evidence_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("equals_law".to_string(), "evidence_id is not in wrapper".to_string()))
		};
		if !wrapper.is_nexq() {
			return Err(SRLError("equals_law".to_string(), "wrapper is no nexq-wrapper".to_string()));
		}
		let evi_cell = match evidence_path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let (a, b) = match evi_cell.get_equals_cell_arguments() {
			Ok((x, y)) => (x, y),
			Err(srl_error) => return Err(srl_error)
		};

		if !wrapper.is_around(&src_path) {
			return Err(SRLError("equals_law".to_string(), "src_id and evidence_id are not in the same wrapper".to_string()));
		}

		let src_cell = match src_path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let new : Cell;
		if a == src_cell {
			new = b;
		} else if b == src_cell {
			new = a;
		} else {
			return Err(SRLError("equals_law".to_string(), "replace cell does not occur in evidence".to_string()));
		}

		let rule = match src_path.replace_by(new) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		self.add_rule(rule)
	}

	// src_id = "The cell that has to be replaced" | `{0 [=> (= p q) (<p> 0)]}.`
	// evidence_id = "the equals cell"		  | `{0 [=> <(= p q)> (p 0)]}`
	pub fn equals_law_impl(&mut self, src_id : CellID, evidence_id : CellID) -> Result<Cell, SRLError> {
		let src_path = match src_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let evidence_path = match evidence_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		// check whether evidence_id is the condition of a case-cell
		if evidence_id.get_indices().last() != Some(&(0 as usize)) {
			return Err(SRLError("equals_law_impl".to_string(), "evidence_id can't be condition of case-cell".to_string()));
		}
		if let Ok(Cell::Case{..}) = evidence_path.get_parent().get_cell() {} else {
			return Err(SRLError("equals_law_impl".to_string(), "evidence_id can't be condition of case-cell (2)".to_string()));
		}

		let rule_id = src_id.get_rule_id();
		if rule_id != evidence_id.get_rule_id() {
			return Err(SRLError("equals_law_impl".to_string(), "src_id and evidence_id are not in the same rule".to_string()));
		}
		let wrapper = match evidence_path.get_parent().get_child(1).get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("equals_law_impl".to_string(), "no wrapper!".to_string()))
		};
		if !wrapper.is_around(&src_path) {
			return Err(SRLError("equals_law_impl".to_string(), "evi-wrapper is not around src_id".to_string()));
		}

		let evi_cell = match evidence_path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let (a, b) = match evi_cell.get_equals_cell_arguments() {
			Ok((x, y)) => (x, y),
			Err(srl_error) => return Err(srl_error)
		};

		let src_cell = match src_path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let new : Cell;
		if a == src_cell {
			new = b;
		} else if b == src_cell {
			new = a;
		} else {
			return Err(SRLError("equals_law".to_string(), "replace cell does not occur in evidence".to_string()));
		}

		let rule = match src_path.replace_by(new) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		self.add_rule(rule)
	}

	// id: `<(= 'ok' 'wow')>`
	pub fn inequal_constants(&mut self, id : CellID) -> Result<Cell, SRLError> {
		let path = match id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let cell = match path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let (x, y) = match cell.get_equals_cell_arguments() {
			Ok((x, y)) => (x, y),
			Err(srl_error) => return Err(srl_error)
		};
		if !x.is_constant() {
			return Err(SRLError("inequals_constants".to_string(), "first arg not constant".to_string()));
		}
		if !y.is_constant() {
			return Err(SRLError("inequals_constants".to_string(), "second arg is not constant".to_string()));
		}
		if x == y {
			return Err(SRLError("inequals_constants".to_string(), "both args equal".to_string()));
		}
		let rule = match path.replace_by(false_cell()) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		self.add_rule(rule)
	}

	// cell_id: <ok> => (= 'true' <ok>)
	pub fn add_eqt(&mut self, cell_id : CellID) -> Result<Cell, SRLError> {
		let cell_path = match cell_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		if !cell_path.is_bool() {
			return Err(SRLError("add_eqt".to_string(), "cell is not bool".to_string()));
		}

		let cell = match cell_path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let rule = match cell_path.replace_by(equals_cell(true_cell(), cell)) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		self.add_rule(rule)
	}

	// cell_id: (= 'true' <ok>) => <ok>
	pub fn rm_eqt(&mut self, cell_id : CellID) -> Result<Cell, SRLError> {
		let cell_path = match cell_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let cell = match cell_path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let parent_path = cell_path.get_parent(); // XXX crashes, if thats a root cell
		let parent_cell = match parent_path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let (a, b) : (Cell, Cell);
		if let Ok((x, y)) = parent_cell.get_equals_cell_arguments() {
			a = x; b = y;
		} else {
			return Err(SRLError("rm_eqt".to_string(), "not contained in equals cell".to_string()));
		}

		if a != true_cell() {
			return Err(SRLError("rm_eqt".to_string(), "first cell in equals is not 'true'".to_string()));
		}

		if b != cell {
			return Err(SRLError("rm_eqt".to_string(), "second cell in equals is not cell_id".to_string()));
		}

		let rule = match parent_path.replace_by(cell) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let tmp_cell_path = CellPath::create(rule.clone(), parent_path.get_indices());
		if !tmp_cell_path.is_bool() {
			return Err(SRLError("rm_eqt".to_string(), "result is no bool-cell".to_string()));
		}

		self.add_rule(rule)
	}

	pub fn scope_insertion(&mut self, scope_id : CellID, secure : SecureCell) -> Result<Cell, SRLError> {
		let scope_path = match scope_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let (id, body) : (u32, Cell) = match scope_path.get_cell() {
			Ok(Cell::Scope { id : x, body : y }) => (x, *y),
			Ok(_) => return Err(SRLError("scope_insertion".to_string(), "scope_id does not represent scope".to_string())),
			Err(srl_error) => return Err(srl_error)
		};
		let child_path = scope_path.get_child(0);
		if !child_path.is_complete_bool() {
			return Err(SRLError("scope_insertion".to_string(), "body is no complete bool cell".to_string()));
		}
		let wrapper = match scope_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("scope_insertion".to_string(), "no wrapper".to_string()))
		};
		if !wrapper.is_positive() {
			return Err(SRLError("scope_insertion".to_string(), "wrapper is not positive".to_string()));
		}

		let mut highest_id = scope_path.get_root_cell().recurse::<i32>(-1, find_highest);
		let id_amount_in_secure = match secure.get_cell().get_normalized() {
			Ok(x) => x.recurse::<i32>(-1, find_highest) + 1,
			Err(srl_error) => return Err(srl_error)
		};

		let mut path = CellPath::create(body.clone(), vec![]);

		loop {
			loop {
				match path.get_cell() {
					Ok(x) => if x.count_subcells() == 0 { break; },
					Err(_) => panic!("scope_insertion: should not happen!")
				}
				let mut indices = path.get_indices();
				indices.push(0);
				path = CellPath::create(path.get_root_cell(), indices);
			}
			if let Ok(Cell::Var { id : id_out }) = path.get_cell() {
				if id_out == id {
					let normalized = match secure.get_cell().get_normalized_from((highest_id + 1) as u32) {
						Ok(x) => x,
						Err(srl_error) => return Err(srl_error)
					};
					let replaced = match path.replace_by(normalized) {
						Ok(x) => x,
						Err(srl_error) => return Err(srl_error)
					};
					path = CellPath::create(replaced, path.get_indices());
					highest_id += id_amount_in_secure;
				}
			}
			loop {
				let mut indices = path.get_indices();
				if indices.is_empty() {
					let rule = match scope_path.replace_by(path.get_root_cell()) {
						Ok(x) => x,
						Err(srl_error) => return Err(srl_error)
					};
					return self.add_rule(rule);
				} else {
					let len = indices.len();
					let last = indices.remove(len - 1);
					path = CellPath::create(path.get_root_cell(), indices.clone());
					let path_cell = match path.get_cell() {
						Ok(x) => x,
						Err(srl_error) => return Err(srl_error)
					};
					if index_in_len(last + 1, path_cell.count_subcells()) {
						indices.push(last + 1);
						path = CellPath::create(path.get_root_cell(), indices);
						break;
					}
				}
			}
		}
	}

	pub fn scope_creation(&mut self, scope_id : CellID, indices : Vec<Vec<usize>>) -> Result<Cell, SRLError> {
		let mut scope_path = match scope_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let new_id : u32 = (scope_path.get_root_cell().recurse::<i32>(-1, find_highest) + 1) as u32;
		let cell = match scope_path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		if !scope_path.is_complete_bool() {
			return Err(SRLError("scope_creation".to_string(), "scope_id does not contain a complete bool-cell".to_string()));
		}

		let replaced = match scope_path.replace_by(scope(new_id, cell)) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		scope_path = CellPath::create(replaced, scope_path.get_indices());

		let wrapper = match scope_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("scope_creation".to_string(), "no wrapper".to_string()))
		};
		if wrapper.is_positive() {
			return Err(SRLError("scope_creation".to_string(), "wrapper is positive".to_string()));
		}

		for index in indices.clone() {
			for i in 0..scope_path.get_indices().len() {
				if scope_path.get_indices()[i] != index[i] {
					return Err(SRLError("scope_creation".to_string(), "indices are not in the scope".to_string()))
				}
			}
		}

		for i in 0..indices.len()-1 {
			let cell1 = match CellPath::create(scope_path.get_root_cell(), indices[i].clone()).get_cell() {
				Ok(x) => x,
				Err(srl_error) => return Err(srl_error)
			};
			let cell2 = match CellPath::create(scope_path.get_root_cell(), indices[i+1].clone()).get_cell() {
				Ok(x) => x,
				Err(srl_error) => return Err(srl_error)
			};
			if cell1 != cell2 { // XXX does not work for cells containing scopes
				return Err(SRLError("scope_creation".to_string(), "indices do not represent the same cells".to_string()));
			}
		}

		for index in indices {
			let tmp_path = CellPath::create(scope_path.get_root_cell(), index);
			let new_cell = match tmp_path.replace_by(var(new_id)) {
				Ok(x) => x,
				Err(srl_error) => return Err(srl_error)
			};
			scope_path = CellPath::create(new_cell, scope_path.get_indices());
		}
		self.add_rule(scope_path.get_root_cell())
	}

	pub fn implications_derivation(&mut self, case_id : CellID, case_negation_id : CellID) -> Result<Cell, SRLError> {
		let case_path = match case_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let case_negation_path = match case_negation_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let case_cell = match case_path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let case_negation_cell = match case_negation_path.get_cell() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let (case_condition, case_conclusion) = match case_cell {
			Cell::Case { condition : x, conclusion : y} => (*x, *y),
			_ => return Err(SRLError("implications_derivation".to_string(), "case_id does not represent case-cell".to_string()))
		};
		let (case_negation_condition, case_negation_conclusion) = match case_negation_cell {
			Cell::Case { condition : x, conclusion : y} => (*x, *y),
			_ => return Err(SRLError("implications_derivation".to_string(), "case_negation_id does not represent case-cell".to_string()))
		};

		if case_conclusion != case_negation_conclusion {
			return Err(SRLError("implications_derivation".to_string(), "conclusions differ".to_string()));
		}

		if equals_cell(false_cell(), case_condition) != case_negation_condition {
			return Err(SRLError("implications_derivation".to_string(), "conditions are not correct".to_string()));
		}

		let case_wrapper = match case_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("implications_derivation".to_string(), "no wrapper (1)".to_string()))
		};
		let case_negation_wrapper = match case_negation_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("implications_derivation".to_string(), "no wrapper (2)".to_string()))
		};
		if case_wrapper != case_negation_wrapper {
			return Err(SRLError("implications_derivation".to_string(), "different wrappers".to_string()));
		}

		if !case_wrapper.is_nexq() {
			return Err(SRLError("implications_derivation".to_string(), "wrapper contains existance quantor".to_string()));
		}

		if !case_wrapper.is_positive() {
			return Err(SRLError("implications_derivation".to_string(), "wrapper is negative".to_string()));
		}
		self.add_rule(case_conclusion)
	}

	pub fn scope_exchange(&mut self, outer_scope_id : CellID) -> Result<Cell, SRLError> {
		let outer_scope_path = match outer_scope_id.get_path(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let inner_scope_path = outer_scope_path.get_child(0);
		let outer_id = match outer_scope_path.get_cell() {
			Ok(Cell::Scope { id : x, ..}) => x,
			Ok(_) => return Err(SRLError("scope_exchange".to_string(), "outer cell is no scope".to_string())),
			Err(srl_error) => return Err(srl_error)
		};
		let (inner_id, body) = match inner_scope_path.get_cell() {
			Ok(Cell::Scope { id : x, body : y }) => (x, *y),
			Ok(_) => return Err(SRLError("scope_exchange".to_string(), "inner cell is no scope".to_string())),
			Err(srl_error) => return Err(srl_error)
		};

		let rule = match outer_scope_path.replace_by(scope(inner_id, scope(outer_id, body))) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		self.add_rule(rule)
	}
}
