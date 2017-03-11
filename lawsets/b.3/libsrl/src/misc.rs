use cell::Cell;
use error::SRLError;

pub fn index_in_len(index : usize, len : usize) -> bool {
	index < len
}

pub fn true_cell() -> Cell {
	simple_by_str("'true'")
}

pub fn false_cell() -> Cell {
	simple_by_str("'false'")
}

pub fn equals_cell(cell1 : Cell, cell2 : Cell) -> Cell {
	complex(vec![simple_by_str("="), cell1, cell2])
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

impl Cell {
	pub fn get_equals_cell_arguments(&self) -> Result<(Cell, Cell), SRLError> {
		if let &Cell::Complex { cells : ref cells_out } = self {
			if cells_out.len() != 3 {
				return Err(SRLError("get_equals_cell_arguments".to_string(), "complex cell does not have 3 arguments".to_string()));
			}
			if cells_out[0] != simple_by_str("=") {
				return Err(SRLError("get_equals_cell_arguments".to_string(), "first cell is not =".to_string()));
			}
			return Ok((cells_out[1].clone(), cells_out[2].clone()));
		} else {
			return Err(SRLError("get_equals_cell_arguments".to_string(), "cell is not complex".to_string()));
		}
	}
}
