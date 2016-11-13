use cell::Cell;

pub trait Evidence {
	fn first(&self) -> &Cell;
	fn second(&self) -> &Cell;

	fn is_valid(&self) -> bool {
		self.first().is_valid() && self.second().is_valid()
	}

	fn cmp(&self, other : &Evidence) -> i32 {
		let mut matches = 0;
		if self.first() == other.first() || self.first() == other.second() { matches += 1; }
		if self.second() == other.first() || self.second() == other.second() { matches += 1; }
		matches
	}

	fn first_cloned(&self) -> Cell {
		self.first().clone()
	}

	fn second_cloned(&self) -> Cell {
		self.second().clone()
	}

	fn equals(&self, other : &Evidence) -> bool {
		self.cmp(other) == 2
	}
}
