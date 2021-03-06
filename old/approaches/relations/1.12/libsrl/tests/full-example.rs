extern crate libsrl;

use libsrl::db::Database;
use libsrl::interface::equals_evi::EqualsEvidence;
use libsrl::cell::mani::simple_by_str;

#[test]
fn full_example () {
	let mut db = match Database::by_string("") {
		Ok(x) => x,
		Err(y) => panic!(format!("ERR1={}", y))
	};
	let evi : EqualsEvidence = match db.equals_evi_i().single_cell(simple_by_str("a")) {
		Ok(x) => x,
		Err(y) => panic!(format!("ERR2={}", y))
	};
	match db.apply_i().create_equals_rule(evi) {
		Err(x) => panic!(format!("ERR3={}", x)),
		Ok(_) => {}
	}
	assert_eq!(&db.get_rule(0).to_rule_string(), "equals a a.");
	assert_eq!(&db.get_rule(0).to_string(), "(equals a a)");
	assert_eq!(&db.get_rule(0).to_unwrapped_string(), "equals a a");
}
