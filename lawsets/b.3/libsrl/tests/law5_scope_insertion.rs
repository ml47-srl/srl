extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::secure::SecureCell;

#[test]
fn test_scope_insertion() {
	let mut db = match Database::by_string("{0 (= 'true' (p 0)) }.") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![]);
	let secure = match SecureCell::by_string("{0 (= 0 0)}") {
		Ok(x) => x,
		Err(_) => panic!("panic! (2)")
	};

	match db.scope_insertion(cell_id, secure) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "= 'true' (p {0 (= 0 0)})."); }
		Err(srl_error) => panic!("panic! (3) err: {:?}", srl_error)
	}
}
