extern crate bot;
use bot::libsrl;
pub extern crate time;
mod fs;
mod proof;
mod room;
use std::env;

fn main() {
	let args : Vec<String> = env::args().collect();
	let ref proofspath = args[1];
	room::execute(proofspath);
}
