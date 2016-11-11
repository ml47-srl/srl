use cell::Cell;
use evi::EqualsEvidence;
use evi::DifferenceEvidence;

pub struct ParadoxInterface<'a>(pub &'a Vec<Cell>);

impl<'a> ParadoxInterface<'a> {
	fn on_paradox(&self) {
		println!("The Database is paradox. Something has gone wrong here..");
		panic!("PARADOX!");
	}

	pub fn evidence_paradox_equal_and_differ(&self, equals_evidence : &EqualsEvidence, differ_evidence : &DifferenceEvidence) -> Result<(), String> {
		if (equals_evidence.0 == differ_evidence.0 && equals_evidence.1 == differ_evidence.1) || (equals_evidence.0 == differ_evidence.1 && equals_evidence.1 == differ_evidence.0) {
			self.on_paradox();
			Ok(())
		} else {
			Err("Database::evidence_paradox_equal_and_differ(): wrong cells".to_string())
		}
	}

}
