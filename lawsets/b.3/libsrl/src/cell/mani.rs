use cell::Cell;

pub fn true_cell() -> Cell {
	simple_by_str("'true'")
}

pub fn false_cell() -> Cell {
	simple_by_str("'false'")
}

pub fn equals_cell(cell1 : Cell, cell2 : Cell) -> Cell {
	complex(vec![simple_by_str("equals"), cell1, cell2])
}

pub fn destructure_equals_cell(equals_cell : Cell) -> Result<(Cell, Cell), String> {
	if ! equals_cell.is_valid() {
		return Err("equals_cell is invalid".to_string());
	}

	match equals_cell {
		Cell::Complex { cells : cells_out } => {
			if cells_out.len() != 3 {
				return Err("equals_cell should have 2 arguments".to_string());
			}

			match cells_out[0] {
				Cell::Simple { string : ref string_out } => {
					if string_out != "equals" {
						return Err("first equals argument is not \"equals\"".to_string());
					}
				},
				_ => return Err("first argument of equals cell isn't simple ".to_string()),
			}

			return Ok((cells_out[1].clone(), cells_out[2].clone()));
		}
		_ => return Err("equals_cell is no complex_cell".to_string()),
	}
}

pub fn simple(string_arg : String) -> Cell {
	Cell::Simple { string : string_arg }
}

pub fn simple_by_str(string_arg : &str) -> Cell {
	Cell::Simple { string : string_arg.to_string() }
}

pub fn complex(cells_arg : Vec<Cell>) -> Cell {
	if cells_arg.len() < 2 {
		panic!("complex cell neeeds more than 1 argument");
	}
	Cell::Complex { cells : cells_arg }
}

pub fn scope(id : u32, body : Cell) -> Cell {
	return Cell::Scope { id : id, body : Box::new(body) };
}

pub fn var(id: u32) -> Cell {
	return Cell::Var { id : id };
}

pub fn case(condition : Cell, conclusion : Cell) -> Cell {
	return Cell::Case { condition : Box::new(condition), conclusion : Box::new(conclusion) };
}
