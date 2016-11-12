extern crate libsrl;

use libsrl::db::Database;

#[test]
fn main() {
	assert_eq!("(equals a b)", Database::by_string("equals a b.").unwrap().get_rule(0).to_string());
	assert_eq!("(equals a b)", Database::by_string("(equals a b).").unwrap().get_rule(0).to_string());
	assert_eq!("(equals a b)", Database::by_string("((equals a b)).").unwrap().get_rule(0).to_string());
	assert_eq!("a", Database::by_string("(a).").unwrap().get_rule(0).to_string());
	assert_eq!("(a b)", Database::by_string("(a) b.").unwrap().get_rule(0).to_string());
}
