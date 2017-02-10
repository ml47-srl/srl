use cell::Cell;
use navi::RuleID;
use evi::Evidence;
use misc::*;

pub struct DifferEvidenceInterface<'a>(&'a Vec<Cell>);

impl<'a> DifferEvidenceInterface<'a> {
	pub fn new(x : &'a Vec<Cell>) -> DifferEvidenceInterface {
		DifferEvidenceInterface(x)
	}

	// R12.2
	// equals 'false' (equals a b)
	pub fn equals_false_rule(&self, rule_id : RuleID) -> Result<DifferEvidence, String> {
		if ! rule_id.is_valid(&self.0) {
			return Err("invalid rule_id".to_string());
		}
		let cell = rule_id.get_cell(&self.0);
		match destructure_equals_cell(cell) {
			Ok((cell1, cell2)) => {

				let othercell : Cell;
				let false_cell = false_cell();

				if cell1 == false_cell {
					othercell = cell2;
				} else if cell2 == false_cell {
					othercell = cell1;
				} else {
					return Err("there is no false cell".to_string());
				}

				match destructure_equals_cell(othercell) {
					Ok((cell1, cell2)) => return Ok(DifferEvidence(cell1, cell2)),
					Err(x) => return Err(x)
				}
			},
			Err(x) => return Err(x)
		}
	}

	// R4
	pub fn constants(&self, cell1 : Cell, cell2 : Cell) -> Result<DifferEvidence, String> {
		if ! cell1.is_valid() {
			return Err("cell1 is invalid".to_string());
		}
		if ! cell2.is_valid() {
			return Err("cell2 is invalid".to_string());
		}
		if ! cell1.is_constant() {
			return Err("cell1 is no constant".to_string());
		}
		if ! cell2.is_constant() {
			return Err("cell2 is no constant".to_string());
		}
		if cell1 == cell2 {
			return Err("cell1 and cell2 are equals".to_string());
		}

		return Ok(DifferEvidence(cell1, cell2));
	}
}

pub struct DifferEvidence(Cell, Cell);

impl Evidence for DifferEvidence {
	fn first(&self) -> &Cell {
		&self.0
	}

	fn second(&self) -> &Cell {
		&self.1
	}
}
