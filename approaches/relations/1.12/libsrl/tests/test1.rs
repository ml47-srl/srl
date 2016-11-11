extern crate libsrl;

use libsrl::Database;

#[test]
fn main() {
	assert_eq!("(equals a b)", Database::by_string("equals a b.").get_rule(0).to_string());
	assert_eq!("(equals a b)", Database::by_string("(equals a b).").get_rule(0).to_string());
	assert_eq!("(equals a b)", Database::by_string("((equals a b)).").get_rule(0).to_string());
	assert_eq!("a", Database::by_string("(a).").get_rule(0).to_string());
	assert_eq!("(a b)", Database::by_string("(a) b.").get_rule(0).to_string());
}
