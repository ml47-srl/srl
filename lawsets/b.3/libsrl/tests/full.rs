extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::secure::SecureCell;

#[test]
fn test_full() {
	let mut db = match Database::by_string("(= x y). {0 (p x)}. {0 (= (this 0) 0)}.") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	assert_eq!(db.get_rule(1).to_rule_string(), "= x y.");
	assert_eq!(db.get_rule(2).to_rule_string(), "{0 (p x)}.");
	assert_eq!(db.get_rule(3).to_rule_string(), "{0 (= (this 0) 0)}.");
	let evi = CellID::create(1, vec![]);
	let src = CellID::create(2, vec![0, 1]);
	match db.equals_law(src, evi) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "{0 (p y)}."); }
		Err(_) => panic!("failure!")
	};
	assert_eq!(db.get_rule(4).to_rule_string(), "{0 (p y)}.");


	let secure = match SecureCell::by_string("(a b)") {
		Ok(x) => x,
		Err(_) => panic!("error!")
	};

	let blub = CellID::create(3, vec![]);
	match db.scope_insertion(blub, secure) {
		Ok(x) => println!("rule: {}", x.to_rule_string()),
		Err(srl_error) => panic!("panic: {:?}", srl_error)
	}
}
