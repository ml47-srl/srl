use cell::Cell;
use navi::RuleID;
use evi::Evidence;
use misc::*;

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
		return match destructure_equals_cell(cell) {
			Ok((cell1, cell2)) => return Ok(EqualsEvidence(cell1, cell2)),
			Err(x) => Err(x)
		}
	}

	// R5
	pub fn true_rule(&self, rule_id : &RuleID) -> Result<EqualsEvidence, String> {
		if ! rule_id.is_valid(self.0) {
			return Err("rule_id is invalid".to_string());
		}
		Ok(EqualsEvidence(rule_id.get_cell(self.0), true_cell()))
	}

	// R6
	// (a = b) && (b = c) => (a = c)
	pub fn transitive(&self, evi1 : EqualsEvidence, evi2 : EqualsEvidence) -> Result<EqualsEvidence, String> {
		if ! evi1.is_valid() {
			return Err("evidence 1 is invalid".to_string());
		}
		if ! evi2.is_valid() {
			return Err("evidence 2 is invalid".to_string());
		}
		if evi1.equals(&evi2) {
			return Err("both evidences match".to_string());
		}

		if evi1.first() == evi2.first() {
			return Ok(EqualsEvidence(evi1.second_cloned(), evi2.second_cloned()));
		} else if evi1.first() == evi2.second() {
			return Ok(EqualsEvidence(evi1.second_cloned(), evi2.first_cloned()));
		} else if evi1.second() == evi2.second() {
			return Ok(EqualsEvidence(evi1.first_cloned(), evi2.first_cloned()));
		} else if evi1.second() == evi2.first() {
			return Ok(EqualsEvidence(evi1.first_cloned(), evi2.second_cloned()));
		} else {
			return Err("both evidences have nothing in common".to_string());
		}
	}
}

pub struct EqualsEvidence(Cell, Cell);

impl Evidence for EqualsEvidence {
	fn first(&self) -> &Cell {
		&self.0
	}

	fn second(&self) -> &Cell {
		&self.1
	}
}
