extern crate srl_bot;
use srl_bot::bot::Bot;
use srl_bot::libsrl::db::Database;
use srl_bot::libsrl::gen::equals_cell;
use srl_bot::libsrl::gen::simple_by_str;

#[test]
fn test_1() {
	let mut db = Database::by_string("").unwrap();
	let mut bot = Bot::gen();
	println!("did? {}", bot.proof(&equals_cell(simple_by_str("a"), simple_by_str("a")), &mut db));
}
