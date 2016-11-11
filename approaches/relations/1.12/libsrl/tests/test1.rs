extern crate libsrl;

#[test]
fn main() {
	assert_eq!("(equals a b)", libsrl::Database::by_string("equals a b.").get_rule(0).to_string());
}
