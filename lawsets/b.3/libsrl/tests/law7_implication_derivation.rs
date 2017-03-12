/*
7. Implikations-ableitung
   Sei W(x) ein positiver nexq-Wrapper.
   Wenn
	W("`[=> [[bool]] [[cell]]]`")
   und
	W("`[=> (= 'false' [[bool]]) [[cell]]]`"), dann gilt:
   W("`[[cell]]`")
   [[bool]] muss dabei eine bool-zelle sein.
*/

extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::secure::SecureCell;

#[test]
fn test_implications_derivation() {
	let mut db = match Database::by_string("(= 'true' y).") {
		Ok(x) => x,
		Err(_) => panic!("database failure!")
	};

	let cell_id = CellID::create(1, vec![]);
	let secure = match SecureCell::by_string("(= 'true' x)") {
		Ok(x) => x,
		Err(_) => panic!("panic! (2)")
	};

	match db.case_creation(cell_id.clone(), secure) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "[=> (= 'true' x) (= 'true' y)]."); }
		Err(srl_error) => panic!("panic! (3) err: {:?}", srl_error)
	}

	let secure = match SecureCell::by_string("(= 'false' (= 'true' x))") {
		Ok(x) => x,
		Err(_) => panic!("panic! (4)")
	};

	match db.case_creation(cell_id, secure) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "[=> (= 'false' (= 'true' x)) (= 'true' y)]."); }
		Err(srl_error) => panic!("panic! (5) err: {:?}", srl_error)
	}

	let cell_id1 = CellID::create(2, vec![]);
	let cell_id2 = CellID::create(3, vec![]);
	match db.implications_derivation(cell_id1, cell_id2) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "= 'true' y."); }
		Err(_) => panic!("panic! (6)")
	}
}
