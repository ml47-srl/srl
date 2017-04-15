use std::path::{Path, PathBuf};

use fs::*;
use bot::Bot;
use proof::Proof;
use libsrl::error::SRLError;

fn get_proof(proofspath : &Path, i : i32) -> Option<Proof> {
	let pbuf = proofspath.join("p".to_string() + &i.to_string());
	let string = read_file(pbuf.as_path()).unwrap();
	match Proof::from_string(string) {
		Ok(x) => return Some(x),
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

fn add_res(botname : &str, instance : u32, data : ()) { // TODO make data useful
	panic!("TODO")
}

pub fn execute(proofspath : &str) {
	let proofspath = Path::new(proofspath);
	// TODO
}
