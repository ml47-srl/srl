pub extern crate libsrl;
pub extern crate serde;

#[macro_use]
pub extern crate serde_derive;

pub extern crate serde_json;
pub extern crate rand;
pub extern crate time;

use libsrl::gen::{equals_cell, simple_by_str};
use libsrl::db::Database;

mod proof;
use proof::Proof;
mod room;
use room::Room;
mod bot;
use bot::LinBot;

fn main() {
	let mut room = Room::new();
	room.add_bot(Box::new(LinBot::gen()));

	let a = simple_by_str("a");
	let target = equals_cell(a.clone(), a);
	let db = Database::by_string("").expect("empty database creation failed");

	room.add_proof(Proof::create(target, db));

	room.run();
}
