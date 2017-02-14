use super::Database;
use cell::Cell;
use error::SRLError;
use navi::CellID;

impl Database {
	// change_id = "The cell that has to be replaced" | `{0 ([p] 0)}.`
	// evidence_id = "the equals cell"		  | `{0 [(= p q)]}`
	pub fn equals_law(&mut self, src_id : CellID, evidence_id : CellID) -> Result<Cell, SRLError> {
		match evidence_id.get_parent() {
			Ok(wrapper) => {
				if !wrapper.is_nexq_wrapper() {
					return Err(SRLError("equals_law".to_string(), "wrapper is no nexq-wrapper".to_string()));
				}
				match evidence_id.get_cell(&self.rules) {
					Ok(evi_cell) => {
						match evi_cell.get_equals_cell_arguments() {
							Ok((x, y)) => {
								assert!(wrapper.is_around(&src_id));

								match src_id.get_cell(&self.rules) {
									Ok(src_cell) => {
										let new : Cell;
										if x == src_cell {
											new = y;
										} else if y == src_cell {
											new = x;
										} else {
											return Err(SRLError("equals_law".to_string(), "replace cell does not occur in evidence".to_string()));
										}
										match src_id.replace_by(&self.rules, new) {
											Ok(rule) => {
												self.rules.push(rule.clone());
												return Ok(rule);
											},
											Err(srl_error) => return Err(srl_error)
										}
									},
									Err(srl_error) => return Err(srl_error)
								}
							},
							Err(srl_error) => return Err(srl_error)
						}
					}
					Err(srl_error) => return Err(srl_error)
				}
			},
			Err(srl_error) => return Err(srl_error)
		}
	}

	// TODO
	// pub fn equals_law_impl(&mut self /* TODO */) -> Result<Cell, SRLError> {}
}
