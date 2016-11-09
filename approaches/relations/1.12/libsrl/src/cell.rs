pub enum Cell {
	SimpleCell { string : String },
	ComplexCell { cells: Vec<Cell> } 
}

impl Cell {
	pub fn by_string(string : &str) -> Cell {
		panic!("Cell::by_string(): TODO");
	}
}
