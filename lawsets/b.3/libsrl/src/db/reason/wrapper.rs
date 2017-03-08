use navi::CellPath;
use cell::Cell;
use misc::false_cell;

pub struct Wrapper {
	cell_path : CellPath,
	positive : bool,
	nallq : bool,
	nexq : bool
}

impl PartialEq for Wrapper {
	fn eq(&self, wrapper : &Wrapper) -> bool {
		panic!("TODO")
	}
}

impl CellPath {
	pub fn get_wrapper(&self) -> Option<Wrapper> {
		let mut indices = self.get_indices();
		let mut positive : bool = true;
		let mut nallq : bool = true;
		let mut nexq : bool = true;
		let mut cell : Cell = self.get_root_cell();

		while !indices.is_empty() {
			let index : usize = indices.remove(0);
			match cell {
				Cell::Scope {..} => {
					if index != 0 {
						return None
					}
					cell = cell.get_subcell(0);

					if positive {
						nallq = false;
					} else {
						nexq = false;
					}
				},
				Cell::Case {..} => {
					if index != 1 { // only the second arg shall be 'in wrapper'
						return None
					}
				},
				Cell::Complex {..} => { // only = 'false' is allowed here!
					let (x, y) = match cell.get_equals_cell_arguments() {
						Ok((x, y)) => (x, y),
						_ => return None
					};
					if index == 0 {
						if y == false_cell() {
							cell = x;
							positive = !positive;
						} else {
							return None
						}
					} else if index == 1 {
						if x == false_cell() {
							cell = y;
							positive = !positive;
						} else {
							return None
						}
					} else {
						return None
					}
				},
				_ => return None
			}
		}
		Some(Wrapper {cell_path : self.clone(), positive : positive, nallq : nallq, nexq : nexq})
	}
}

impl Wrapper {
	pub fn is_positive(&self) -> bool { self.positive }
	pub fn is_nallq(&self) -> bool { self.nallq }
	pub fn is_nexq(&self) -> bool { self.nexq }

	pub fn is_around(&self, path : &CellPath) -> bool {
		let indices1 = self.cell_path.get_indices();
		let indices2 = path.get_indices();

		let mut cell1 = match self.cell_path.get_cell() {
			Ok(x) => x,
			Err(_) => panic!("Wrapper::is_around(): should not happen!")
		};
		let mut cell2 = match path.get_cell() {
			Ok(x) => x,
			Err(_) => panic!("Wrapper::is_around(): should not happen! (2)")
		};

		let tmp_cell : Cell = false_cell(); // any cell ..

		for i in 0..indices1.len() {
			let index : usize = indices1[i];
			if index != indices2[i] {
				return false
			}

			if cell1.with_subcell(tmp_cell.clone(), index) != cell2.with_subcell(tmp_cell.clone(), index) {
				return false
			}

			cell1 = cell1.get_subcell(index);
			cell2 = cell2.get_subcell(index);
		}
		true
	}

	/* // needed?
		pub fn is_directly_around(&self, id : CellID) -> bool {
			false
		}
	*/
}
