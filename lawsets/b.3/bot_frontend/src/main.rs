extern crate srl_bot;
use srl_bot::bot::Bot;
use srl_bot::libsrl::db::Database;
use srl_bot::libsrl::gen::equals_cell;
use srl_bot::libsrl::gen::simple_by_str;
use srl_bot::libsrl::cell::Cell;

fn main() {
	let mut bot : Bot = Bot::from_file("default.bot").expect("failed loading bot from file");
	let mut db : Database = Database::by_string("").expect("failed generating database");
	let target : Cell = equals_cell(simple_by_str("a"), simple_by_str("a"));
	let result = bot.practice(&target, &mut db);
	println!("practice worked: {}", result);
	bot.to_file("default.bot").expect("failed writing in the bot file");
}
