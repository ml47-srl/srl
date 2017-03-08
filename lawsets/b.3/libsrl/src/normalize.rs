use error::SRLError;
use cell::Cell;
use misc::*;

fn get_new_id(old_id : u32, scope_ids : &Vec<u32>, in_scope_vec : &Vec<bool>) -> Result<u32, SRLError> {
	for index in 0..scope_ids.len() {
		if old_id == scope_ids[index as usize] {
			if in_scope_vec[index] {
				return Ok(index as u32);
			} else {
				return Err(SRLError("get_new_id".to_string(), format!("already out of scope with id '{}'", old_id)));
			}
		}
	}
	return Err(SRLError("get_new_id".to_string(), format!("id '{}' is not in scope_ids '{:?}'", old_id, scope_ids)));
}

#[test]
fn test_get_normalized() {
	if let Ok(_) = complex(vec![var(0), scope(0, simple_by_str("ok"))]).get_normalized() { panic!("test_get_normalized(): should not accept (0)"); }
	if let Ok(_) = complex(vec![scope(0, simple_by_str("ok")), var(0)]).get_normalized() { panic!("test_get_normalized(): should not accept (1)"); }
	if let Ok(_) = scope(0, scope(0, simple_by_str("wow"))).get_normalized() { panic!("test_get_normalized(): should not accept (2)"); }
	assert_eq!(scope(0, scope(1, var(1))).to_string(), scope(1, scope(2, var(2))).get_normalized().unwrap().to_string());
}


impl Cell {
       // creates new cell with normalized scopes
       // -- errors on var out of scope/multiple scopes with same id

	pub fn get_normalized(&self) -> Result<Cell, SRLError> {
		return self.get_normalized_from_r(&mut vec![], &mut vec![], 0);
	}

	pub fn get_normalized_from(&self, from : u32) -> Result<Cell, SRLError> {
		return self.get_normalized_from_r(&mut vec![], &mut vec![], from);
	}


	fn get_normalized_from_r(&self, vec : &mut Vec<u32>, in_scope_vec : &mut Vec<bool>, from : u32) -> Result<Cell, SRLError> { // XXX detangle matches
		match &self {
			&&Cell::Simple { string : ref string_out } => {
				return Ok(simple(string_out.to_string()));
			}
			&&Cell::Complex { cells : ref cells_out } => {
				let mut new_cells = Vec::new();
				for cell in cells_out {
					match cell.get_normalized_from_r(vec, in_scope_vec, from) {
						Ok(new_cell) => {
							new_cells.push(new_cell);
						}
						Err(srl_error) => return Err(srl_error)
					}
				}
				return Ok(complex(new_cells));
			}
			&&Cell::Scope { id : id_out, body : ref body_out } => {
				if vec.contains(&id_out) {
					return Err(SRLError("get_normalized_from_r".to_string(), format!("id '{}' used twice", id_out)));
				}

				vec.push(id_out);
				in_scope_vec.push(true);
				let new_id = (vec.len() - 1) as u32 + from;

				match body_out.get_normalized_from_r(vec, in_scope_vec, from) {
					Ok(new_body) => {
						in_scope_vec.pop();
						in_scope_vec.push(false);
						return Ok(scope(new_id, new_body));
					}
					Err(srl_error) => return Err(srl_error)
				}
			}
			&&Cell::Var { id : id_out } => {
				match get_new_id(id_out, vec, in_scope_vec) {
					Ok(new_id) => return Ok(Cell::Var { id : new_id + from }),
					Err(srl_error) => return Err(srl_error)
				}
			}
			&&Cell::Case { condition : ref condition_out, conclusion : ref conclusion_out } =>  {
				match condition_out.get_normalized_from_r(vec, in_scope_vec, from) {
					Ok(condition_new) => {
						match conclusion_out.get_normalized_from_r(vec, in_scope_vec, from) {
							Ok(conclusion_new) => {
								return Ok(case(condition_new, conclusion_new));
							}
							Err(srl_error) => return Err(srl_error)
						}
					}
					Err(srl_error) => return Err(srl_error)
				}
			}
		}
	}
}

