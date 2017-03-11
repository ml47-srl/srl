extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::secure::SecureCell;

#[test]
fn test_case_creation() {
	let mut db = match Database::by_string("= 'true' y.") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![]);
	let secure = match SecureCell::by_string("(= 'true' x)") {
		Ok(x) => x,
		Err(_) => panic!("panic! (2)")
	};

	match db.case_creation(cell_id, secure) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "[=> (= 'true' x) (= 'true' y)]."); }
		Err(srl_error) => panic!("panic! (3) err: {:?}", srl_error)
	}
}
