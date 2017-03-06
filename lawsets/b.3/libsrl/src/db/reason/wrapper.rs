use navi::CellID;
use cell::Cell;
use misc::false_cell;

pub struct Wrapper {
	cell_id : CellID,
	positive : bool,
	nallq : bool,
	nexq : bool
}

impl CellID {
	pub fn get_wrapper(&self, rules : &Vec<Cell>) -> Option<Wrapper> {
		let mut indices = self.get_indices();
		let mut positive : bool = true;
		let mut nallq : bool = true;
		let mut nexq : bool = true;
		let mut cell : Cell;
		cell = match self.get_rule_id().get_cell(rules) {
			Ok(x) => x,
			_ => { println!("CellID::get_wrapper(): some kind of bug probabbly"); return None }
		};

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
		Some(Wrapper {cell_id : self.clone(), positive : positive, nallq : nallq, nexq : nexq})
	}
}

impl Wrapper {
	pub fn is_positive(&self) -> bool { self.positive }
	pub fn is_nallq(&self) -> bool { self.nallq }
	pub fn is_nexq(&self) -> bool { self.nexq }

	pub fn is_around(&self, id : &CellID) -> bool {
		false // TODO
	}

	/* // needed?
		pub fn is_directly_around(&self, id : CellID) -> bool {
			false
		}
	*/
}
