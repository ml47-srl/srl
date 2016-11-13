use cell::Cell;
use evi::EqualsEvidence;
use navi::RuleID;

pub struct EqualsEvidenceInterface<'a>(&'a Vec<Cell>);

impl<'a> EqualsEvidenceInterface<'a> {
	pub fn new(x : &'a Vec<Cell>) -> EqualsEvidenceInterface {
		EqualsEvidenceInterface(x)
	}

	// R3
	pub fn single_cell(&self, cell : Cell) -> Result<EqualsEvidence, String> {
		if cell.is_valid() {
			Ok(EqualsEvidence(cell.clone(), cell))
		} else {
			Err("invalid cell".to_string())
		}
	}

	// R2.2
	// equals a b
	pub fn equals_rule(&self, rule_id : &RuleID) -> Result<EqualsEvidence, String> {
		if ! rule_id.is_valid(&self.0) {
			return Err("rule_id is invalid".to_string());
		}
		let cell = rule_id.get_cell(&self.0);
		return match Cell::destructure_equals_cell(cell) {
			Ok((cell1, cell2)) => return Ok(EqualsEvidence(cell1, cell2)),
			Err(x) => Err(x)
		}
	}

	// R5
	pub fn true_rule(&self, rule_id : &RuleID) -> Result<EqualsEvidence, String> {
		if ! rule_id.is_valid(self.0) {
			return Err("rule_id is invalid".to_string());
		}
		Ok(EqualsEvidence(rule_id.get_cell(self.0), Cell::true_cell()))
	}
}
