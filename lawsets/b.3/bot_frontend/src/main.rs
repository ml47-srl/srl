extern crate srl_bot;
use srl_bot::bot::Bot;
use srl_bot::libsrl::db::Database;
use srl_bot::libsrl::gen::equals_cell;
use srl_bot::libsrl::gen::simple_by_str;

fn main() {
	let mut bot : Bot = Bot::from_file("default.bot").expect("failed loading bot from file");
}
