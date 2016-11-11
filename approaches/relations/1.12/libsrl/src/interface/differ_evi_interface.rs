use cell::Cell;

pub struct DifferEvidenceInterface<'a>(&'a Vec<Cell>);

impl<'a> DifferEvidenceInterface<'a> {
	pub fn new(x : &'a Vec<Cell>) -> DifferEvidenceInterface {
		DifferEvidenceInterface(x)
	}
}
