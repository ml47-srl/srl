pub extern crate libsrl;
pub extern crate serde;

#[macro_use]
pub extern crate serde_derive;

pub extern crate serde_json;
pub extern crate rand;
pub extern crate time;
mod fs;
mod bot;
mod cont;
mod proof;
mod room;
use room::Room;

fn main() {
	let room = Room::init("./room");
	room.tick();
}
