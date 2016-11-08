pub enum Cell {
	SimpleCell { string : String },
	ComplexCell { cells: Vec<Cell> } 
}
