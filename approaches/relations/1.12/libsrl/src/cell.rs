use std::fmt;

static VALID_SIMPLE_CELL_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_=";


#[derive(PartialEq)]
pub enum Cell {
	SimpleCell { string : String },
	ComplexCell { cells: Vec<Cell> } 
}

impl fmt::Debug for Cell {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

impl Cell {
	pub fn to_string(&self) -> String {
		match &self {
			&&Cell::SimpleCell { string : ref x } => x.to_string(),
			&&Cell::ComplexCell { cells : ref x } => {
				let mut s = String::from("(");
				for i in x {
					let tmp_string : String = " ".to_string() + &i.to_string() + " ";
					s.push_str(&tmp_string);
				}
				s.push(')');
				s
			}
		}
	}

	pub fn by_string(string : &str) -> Result<Cell, ()> {
		if string.starts_with("(") {
			return Err(());
		} else {
			for chr in string.chars() {
				if ! VALID_SIMPLE_CELL_CHARS.contains(chr) {
					panic!("inacceptable char '{}' in SimpleCell", chr);
				}
				return Ok(Cell::SimpleCell { string: string.to_string() });
			}
		}
		Err(())
	}
}

#[test]
fn test_cell_by_string() {
	assert_eq!(Cell::SimpleCell { string : "wow".to_string() }, Cell::by_string("wow").unwrap());
}
