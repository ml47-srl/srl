mod wrapper;

use super::Database;
use cell::Cell;
use error::SRLError;
use navi::CellID;

impl Database {
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
		self.rules.push(rule.clone());
		Ok(rule)
	}

	// src_id = "The cell that has to be replaced" | `{0 [=> (= p q) (<p> 0)]}.`
	// evidence_id = "the equals cell"		  | `{0 [=> <(= p q)> (p 0)]}`
	pub fn equals_law_impl(&mut self, src_id : CellID, evidence_id : CellID) -> Result<Cell, SRLError> {
		// check whether evidence_id is the condition of a case-cell
		if evidence_id.get_indices().last() != Some(&(0 as usize)) {
			return Err(SRLError("equals_law_impl".to_string(), "evidence_id can't be condition of case-cell".to_string()));
		}
 		if let Ok(Cell::Case{condition : _, conclusion : _}) = evidence_id.get_parent().get_cell(&self.rules) {} else {
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
		self.rules.push(rule.clone());
		Ok(rule)
	}
}
