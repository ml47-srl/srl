use std::path::Path;

use bot::Bot;
use proof::Proof;
use fs::{read_file, write_file, force_file};
use bot::libsrl::db::Database;

fn get_proof(proofspath : &Path, i : i32) -> Option<Proof> {
	let pbuf = proofspath.join("p".to_string() + &i.to_string());
	match Proof::from_dir(pbuf.as_path()) {
		Ok(x) => Some(x),
		Err(_) => None
	}
}

fn get_proofs(proofspath : &Path) -> Vec<Proof> {
	let mut i = 0;
	let mut vec = Vec::new();
	loop {
		match get_proof(proofspath, i) {
			Some(x) => vec.push(x),
			None => break
		}
		i += 1;
	}
	vec
}

pub fn exec(instancepath_str : &str, proofspath_str : &str) {
	let proofspath = Path::new(proofspath_str);

	let instancepath = Path::new(instancepath_str);
	let botfile_pbuf = instancepath.join("botfile");

	let content = read_file(botfile_pbuf.as_path());
	let mut bot = Bot::by_string(content.unwrap());
	for proof in get_proofs(proofspath) {
		let mut db : Database = (*proof.get_db()).clone();
		bot.practice(proof.get_target(), &mut db);
	}
	for proof in get_proofs(proofspath) {
		let mut db : Database = (*proof.get_db()).clone();
		bot.proof(proof.get_target(), &mut db);
	}
	write_file(botfile_pbuf.as_path(), &bot.to_string());
	// TODO write result
}

pub fn new(instancepath : &str) {
	let instancepath = Path::new(instancepath);
	let botfile_pbuf = instancepath.join("botfile");
	let content = Bot::gen().to_string();
	force_file(botfile_pbuf.as_path(), &content);
}
