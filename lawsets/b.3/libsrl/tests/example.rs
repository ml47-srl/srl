extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[test]
fn test_example() {
	let mut db = match Database::by_string("{0 (= (this 0) 0)}. {0 (= (self 0) 0)}.") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	let evi = CellID::create(1, vec![0]);
	let src = CellID::create(2, vec![0, 2]);
	match db.equals_law(src, evi) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "{0 (= (self 0) (this 0))}."); }
		Err(_) => panic!("failure!")
	};
}
