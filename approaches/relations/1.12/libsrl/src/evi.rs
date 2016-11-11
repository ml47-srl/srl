use cell::Cell;

pub struct EqualsEvidence(pub Cell, pub Cell);
pub struct DifferenceEvidence(pub Cell, pub Cell);

impl PartialEq for EqualsEvidence {
	fn eq(&self, other : &EqualsEvidence) -> bool {
		(self.0 == other.0 && self.1 == other.1) || (self.1 == other.0 && self.0 == other.1)
	}
}

impl PartialEq for DifferenceEvidence {
	fn eq(&self, other : &DifferenceEvidence) -> bool {
		(self.0 == other.0 && self.1 == other.1) || (self.1 == other.0 && self.0 == other.1)
	}
}
