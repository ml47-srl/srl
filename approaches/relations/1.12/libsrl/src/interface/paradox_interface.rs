use cell::Cell;
use evi::EqualsEvidence;
use evi::DifferEvidence;

pub struct ParadoxInterface<'a>(&'a Vec<Cell>);

impl<'a> ParadoxInterface<'a> {
	pub fn new(x : &'a Vec<Cell>) -> ParadoxInterface {
		ParadoxInterface(x)
	}

	fn on_paradox(&self) {
		println!("The Database is paradox. Something has gone wrong here..");
		panic!("PARADOX!");
	}

	pub fn equal_and_differ(&self, equals_evidence : &EqualsEvidence, differ_evidence : &DifferEvidence) -> Result<(), String> {
		if (equals_evidence.0 == differ_evidence.0 && equals_evidence.1 == differ_evidence.1) || (equals_evidence.0 == differ_evidence.1 && equals_evidence.1 == differ_evidence.0) {
			self.on_paradox();
			Ok(())
		} else {
			Err("wrong cells".to_string())
		}
	}

}
