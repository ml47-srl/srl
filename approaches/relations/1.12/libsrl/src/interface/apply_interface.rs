use cell::Cell;
use evi::EqualsEvidence;
use navi::CellID;

pub struct ApplyInterface<'a>(pub &'a mut Vec<Cell>);

impl<'a> ApplyInterface<'a> {
	pub fn create_equals_rule(&mut self, evi : EqualsEvidence) -> Result<Cell, String> {
		let result = Cell::complex(vec![Cell::simple_by_str("equals"), evi.0, evi.1]);
		self.0.push(result.clone());
		Ok(result)
	}

	pub fn equals_replace(&mut self, equals_evidence : &EqualsEvidence, target_cell_id : &CellID) -> Result<Cell, String> { // returns and adds rule
		if target_cell_id.is_valid(&self.0) {
			let cell = target_cell_id.get_cell(&self.0);
			if equals_evidence.0 == cell {
				let c = target_cell_id.replace_by(&self.0, equals_evidence.1.clone());
				self.0.push(c.clone());
				return Ok(c);
			} else if equals_evidence.1 == cell {
				let c = target_cell_id.replace_by(&self.0, equals_evidence.0.clone());
				self.0.push(c.clone());
				return Ok(c);
			} else {
				return Err("wrong member of equals_evidence".to_string());
			}
		} else {
			Err("target_cell_id is invalid".to_string())
		}
	}
}
