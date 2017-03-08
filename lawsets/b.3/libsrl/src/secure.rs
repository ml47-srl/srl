use cell::Cell;
use parse::*;
use parse::assemble::*;
use parse::tokenize::*;
use error::SRLError;

pub struct SecureCell(Cell);

impl SecureCell {
	pub fn get_cell(&self) -> Cell {
		self.0.clone()
	}

	pub fn by_string(string : &str) -> Result<SecureCell, SRLError> {
		if string.contains('.') {
			return Err(SRLError("SecureCell::by_string".to_string(), "string contains '.'".to_string()));
		}
		match find_invalid_char(&string) {
			Some(x) => return Err(SRLError("SecureCell::by_string".to_string(), format!("invalid char '{}'", string.chars().nth(x as usize).unwrap()))),
			None => {}
		}
		let string : String = fix_whitespaces(string);
		let tokens = match tokenize(string) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		if ! check_paren_correctness(tokens.clone()) {
			return Err(SRLError("Database::by_string()".to_string(), "Parens are not correct".to_string()));
		}
		let cell = match assemble(tokens) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		let normalized = match cell.get_normalized() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		Ok(SecureCell(normalized))
	}
}
