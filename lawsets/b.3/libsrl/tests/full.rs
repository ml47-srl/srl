extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::create_cell_id;

#[test]
fn test_full() {
	let mut db = match Database::by_string("(= x y). {0 (p x)}.") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	assert_eq!(db.get_rule(1).to_rule_string(), "= x y.");
	assert_eq!(db.get_rule(2).to_rule_string(), "{0 (p x)}.");
	let evi = create_cell_id(1, vec![]);
	let src = create_cell_id(2, vec![0, 1]);
	match db.equals_law(src, evi) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "{0 (p y)}."); }
		Err(_) => panic!("failure!")
	};
	assert_eq!(db.get_rule(3).to_rule_string(), "{0 (p y)}.");
}
