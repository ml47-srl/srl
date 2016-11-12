use cell::Cell;
use navi::RuleID;
use evi::DifferEvidence;

pub struct DifferEvidenceInterface<'a>(&'a Vec<Cell>);

impl<'a> DifferEvidenceInterface<'a> {
	pub fn new(x : &'a Vec<Cell>) -> DifferEvidenceInterface {
		DifferEvidenceInterface(x)
	}

	// equals 'false' (equals a b)
	pub fn equals_false_rule(&self, rule_id : RuleID) -> Result<DifferEvidence, String> {
		if ! rule_id.is_valid(&self.0) {
			return Err("invalid rule_id".to_string());
		}
		let cell = rule_id.get_cell(&self.0);
		match cell {
			Cell::SimpleCell { string : _ } => return Err("argument is simple cell".to_string()),
			Cell::ComplexCell { cells : cells_out }  => {
				if cells_out.len() != 3 {
					return Err("argument should contain 3 cells \"equals 'false' <cell>\"".to_string());
				}
				if &cells_out[0].to_string() != "equals" {
					return Err("argument[0] is no equals-cell".to_string());
				}
				let othercell : Cell;
				if &cells_out[1].to_string() == "'false'" {
					othercell = cells_out[2].clone();
				} else if &cells_out[2].to_string() == "'false'" {
					othercell = cells_out[1].clone();
				} else {
					return Err("there is no 'false' in here".to_string());
				}

				match othercell {
					Cell::SimpleCell { string : _ } => return Err("inner argument is simple cell".to_string()),
					Cell::ComplexCell { cells : cells_out } => {
						if cells_out.len() != 3 {
							return Err("inner argument should contain 3 cell \"equal".to_string());
						}
						if cells_out[0].to_string() != "equals" {
							return Err("inner argument[0] is no equals cell".to_string());
						}
						return Ok(DifferEvidence(cells_out[1].clone(), cells_out[2].clone()));
					}
				}
			}
		}
	}
}
