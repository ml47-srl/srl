mod wrapper;

use super::Database;
use cell::Cell;
use error::SRLError;
use navi::CellID;
use navi::create_cell_id;
use misc::false_cell;
use misc::true_cell;
use misc::equals_cell;
use secure::SecureCell;

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
		let wrapper = match evidence_id.get_wrapper(&self.rules) {
			Some(x) => x,
			None => return Err(SRLError("equals_law".to_string(), "evidence_id is not in wrapper".to_string()))
		};
		if !wrapper.is_nexq() {
			return Err(SRLError("equals_law".to_string(), "wrapper is no nexq-wrapper".to_string()));
		}
		let evi_cell = match evidence_id.get_cell(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let (a, b) = match evi_cell.get_equals_cell_arguments() {
			Ok((x, y)) => (x, y),
			Err(srl_error) => return Err(srl_error)
		};

		if !wrapper.is_around(&src_id, &self.rules) {
			return Err(SRLError("equals_law".to_string(), "src_id and evidence_id are not in the same wrapper".to_string()));
		}

		let src_cell = match src_id.get_cell(&self.rules) {
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

		let rule = match src_id.replace_by(&self.rules, new) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		self.add_rule(rule)
	}

	// src_id = "The cell that has to be replaced" | `{0 [=> (= p q) (<p> 0)]}.`
	// evidence_id = "the equals cell"		  | `{0 [=> <(= p q)> (p 0)]}`
	pub fn equals_law_impl(&mut self, src_id : CellID, evidence_id : CellID) -> Result<Cell, SRLError> {
		// check whether evidence_id is the condition of a case-cell
		if evidence_id.get_indices().last() != Some(&(0 as usize)) {
			return Err(SRLError("equals_law_impl".to_string(), "evidence_id can't be condition of case-cell".to_string()));
		}
		if let Ok(Cell::Case{..}) = evidence_id.get_parent().get_cell(&self.rules) {} else {
			return Err(SRLError("equals_law_impl".to_string(), "evidence_id can't be condition of case-cell (2)".to_string()));
		}

		let rule_id = src_id.get_rule_id();
		if rule_id != evidence_id.get_rule_id() {
			return Err(SRLError("equals_law_impl".to_string(), "src_id and evidence_id are not in the same rule".to_string()));
		}
		let wrapper = match evidence_id.get_parent().get_child(1).get_wrapper(&self.rules) {
			Some(x) => x,
			None => return Err(SRLError("equals_law_impl".to_string(), "no wrapper!".to_string()))
		};
		if !wrapper.is_around(&src_id, &self.rules) {
			return Err(SRLError("equals_law_impl".to_string(), "evi-wrapper is not around src_id".to_string()));
		}

		let evi_cell = match evidence_id.get_cell(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let (a, b) = match evi_cell.get_equals_cell_arguments() {
			Ok((x, y)) => (x, y),
			Err(srl_error) => return Err(srl_error)
		};

		let src_cell = match src_id.get_cell(&self.rules) {
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

		let rule = match src_id.replace_by(&self.rules, new) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		self.add_rule(rule)
	}

	// id: `<(= 'ok' 'wow')>`
	pub fn inequal_constants(&mut self, id : CellID) -> Result<Cell, SRLError> {
		let cell = match id.get_cell(&self.rules) {
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
		let rule = match id.replace_by(&self.rules, false_cell()) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		self.add_rule(rule)
	}

	// cell_id: <ok> => (= 'true' <ok>)
	pub fn add_eqt(&mut self, cell_id : CellID) -> Result<Cell, SRLError> {
		if !cell_id.is_bool(&self.rules) {
			return Err(SRLError("add_eqt".to_string(), "cell is not bool".to_string()));
		}

		let cell = match cell_id.get_cell(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let rule = match cell_id.replace_by(&self.rules, equals_cell(true_cell(), cell)) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		self.add_rule(rule)
	}

	// cell_id: (= 'true' <ok>) => <ok>
	pub fn rm_eqt(&mut self, cell_id : CellID) -> Result<Cell, SRLError> {
		let cell = match cell_id.get_cell(&self.rules) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let parent_id = cell_id.get_parent(); // XXX crashes, if thats a root cell
		let parent_cell = match parent_id.get_cell(&self.rules) {
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

		let rule = match parent_id.replace_by(&self.rules, cell) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};

		let fake_cell_id = create_cell_id(0, parent_id.get_indices());
		if !fake_cell_id.is_bool(&vec![rule.clone()]) {
			return Err(SRLError("rm_eqt".to_string(), "result is no bool-cell".to_string()));
		}

		self.add_rule(rule)
	}

	pub fn scope_insertion(&mut self, scope_id : CellID, cell : SecureCell) -> Result<Cell, SRLError> {
		let (id, body) = match scope_id.get_cell(&self.rules) {
			Ok(Cell::Scope { id : x, body : y }) => (x, y),
			Ok(_) => return Err(SRLError("scope_insertion".to_string(), "scope_id does not represent scope".to_string())),
			Err(srl_error) => return Err(srl_error)
		};
		let child_id = scope_id.get_child(0);
		if !child_id.is_complete_bool(&self.rules) {
			return Err(SRLError("scope_insertion".to_string(), "body is no complete bool cell".to_string()));
		}
		let wrapper = match scope_id.get_wrapper(&self.rules) {
			Some(x) => x,
			None => return Err(SRLError("scope_insertion".to_string(), "no wrapper".to_string()))
		};
		if !wrapper.is_positive() {
			return Err(SRLError("scope_insertion".to_string(), "wrapper is not positive".to_string()));
		}

		// let h = `determine highest scope/var-id`;
		// replace all occurences of scope_id's var-cell with cell, where you add h+1 to all id's inside of cell.
		// you increase h by the number of different id's in cell, while iterating.
		panic!("TODO")
	}
}