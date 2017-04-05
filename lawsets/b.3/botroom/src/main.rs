pub extern crate libsrl;
pub extern crate serde;

#[macro_use]
pub extern crate serde_derive;

pub extern crate serde_json;
pub extern crate rand;
pub extern crate time;

mod fs;
mod cont;
mod proof;
mod room;
mod bot;

use room::Room;

fn main() {
	let room = Room::init("./room", vec![]);
	room.tick();
}
