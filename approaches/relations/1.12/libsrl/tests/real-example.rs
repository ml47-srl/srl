extern crate libsrl;

use libsrl::Database;
use libsrl::evi::EqualsEvidence;
use libsrl::cell::Cell;

#[test]
fn main() {
	let mut db = Database::by_string("equals x y.");
	let evi : EqualsEvidence = match db.evidence_equals_same_cell(Cell::simple_by_str("a")) {
		Ok(x) => x,
		Err(y) => panic!(format!("ERR1={}", y))
	};
	match db.apply_equals(evi) {
		Err(x) => panic!(format!("ERR2={}", x)),
		Ok(_) => {}
	}
	assert_eq!(&db.get_rule(1).to_rule_string(), "equals a a.");
	assert_eq!(&db.get_rule(1).to_string(), "(equals a a)");
}
