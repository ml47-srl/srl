use cell::Cell;

pub struct EqualsEvidence(pub Cell, pub Cell);
pub struct DifferEvidence(pub Cell, pub Cell);

impl PartialEq for EqualsEvidence {
	fn eq(&self, other : &EqualsEvidence) -> bool {
		(self.0 == other.0 && self.1 == other.1) || (self.1 == other.0 && self.0 == other.1)
	}
}

impl PartialEq for DifferEvidence {
	fn eq(&self, other : &DifferEvidence) -> bool {
		(self.0 == other.0 && self.1 == other.1) || (self.1 == other.0 && self.0 == other.1)
	}
}

impl EqualsEvidence {
	pub fn is_valid(&self) -> bool {
		self.0.is_valid() && self.1.is_valid()
	}
}

impl DifferEvidence {
	pub fn is_valid(&self) -> bool {
		self.0.is_valid() && self.1.is_valid()
	}
}
