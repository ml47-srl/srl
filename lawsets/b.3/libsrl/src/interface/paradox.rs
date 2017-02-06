use cell::Cell;
use interface::equals_evi::EqualsEvidence;
use interface::differ_evi::DifferEvidence;
use evi::Evidence;

pub struct ParadoxInterface<'a>(&'a Vec<Cell>);

impl<'a> ParadoxInterface<'a> {
	pub fn new(x : &'a Vec<Cell>) -> ParadoxInterface {
		ParadoxInterface(x)
	}

	fn on_paradox(&self) {
		println!("The Database is paradox. Something has gone wrong here..");
		panic!("PARADOX!");
	}

	// R13.1
	pub fn equal_and_differ(&self, equals_evidence : &EqualsEvidence, differ_evidence : &DifferEvidence) -> Result<(), String> {
		if equals_evidence.equals(differ_evidence) {
			self.on_paradox();
			Ok(())
		} else {
			Err("wrong cells".to_string())
		}
	}

	// R13.2
	// equals (equals a b) 'fish'
	// TODO pub fn malformed_equals(&self, rule_id : &RuleID) -> Result<(), String> { }
}
