use navi::CellID;

pub struct Wrapper {
	id : CellID,
	positive : bool,
	nallq : bool,
	nexq : bool
}

impl CellID {
	pub fn get_wrapper(&self) -> Option<Wrapper> {
		None // TODO
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
