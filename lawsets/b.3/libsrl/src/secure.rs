use cell::Cell;

pub struct SecureCell {
	cell : Cell
}

impl SecureCell {
	pub fn get_cell(&self) -> Cell {
		self.cell.clone()
	}
}
