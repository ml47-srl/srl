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
		match cell {
			Cell::SimpleCell { string : _ } => return Err("rule_id points to simple cell".to_string()),
			Cell::ComplexCell { cells : cells_out } => {
				if cells_out.len() != 3 {
					return Err(format!("rule_id points to cell with {} arguments", cells_out.len()));
				}
				if cells_out[0].to_string() != "equals" {
					return Err(format!("rule_id points to cell which starts with '{}'", cells_out[0]));
				}

				return Ok(EqualsEvidence(cells_out[1].clone(), cells_out[2].clone()));
			}
		}
	}

	// R5
	pub fn true_rule(&self, rule_id : &RuleID) -> Result<EqualsEvidence, String> {
		if ! rule_id.is_valid(self.0) {
			return Err("rule_id is invalid".to_string());
		}
		Ok(EqualsEvidence(rule_id.get_cell(self.0), Cell::simple_by_str("'true'")))
	}
}
