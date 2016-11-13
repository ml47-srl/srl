use cell::Cell;
use evi::EqualsEvidence;
use evi::DifferEvidence;
use navi::CellID;

pub struct ApplyInterface<'a>(&'a mut Vec<Cell>);

impl<'a> ApplyInterface<'a> {
	pub fn new(x : &'a mut Vec<Cell>) -> ApplyInterface {
		ApplyInterface(x)
	}

	// R2.1
	pub fn create_equals_rule(&mut self, evi : EqualsEvidence) -> Result<Cell, String> {
		if ! evi.is_valid() {
			return Err("evidence is invalid".to_string());
		}
		let result = Cell::equals_cell(evi.0, evi.1);
		self.0.push(result.clone());
		Ok(result)
	}

	// R1
	pub fn equals_replace(&mut self, equals_evidence : &EqualsEvidence, target_cell_id : &CellID) -> Result<Cell, String> { // returns and adds rule
		if ! equals_evidence.is_valid() {
			return Err("evidence is invalid".to_string());
		}
		if ! target_cell_id.is_valid(&self.0) {
			return Err("target_cell_id is invalid".to_string());
		}

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
	}

	// R12.1
	pub fn create_equals_false_rule(&mut self, evi : DifferEvidence) -> Result<Cell, String> {
		if ! evi.is_valid() {
			return Err("evidence is invalid".to_string());
		}
		let result = Cell::equals_cell(Cell::equals_cell(evi.0, evi.1), Cell::false_cell());
		self.0.push(result.clone());
		Ok(result)
	}
}
