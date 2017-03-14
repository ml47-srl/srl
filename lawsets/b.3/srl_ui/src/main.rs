extern crate libsrl;
use libsrl::db::Database;

fn main() {
	let db = Database::by_string("").unwrap();
}
