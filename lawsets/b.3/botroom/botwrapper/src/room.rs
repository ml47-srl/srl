use std::path::Path;

use proof::Proof;

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

fn add_res(botname : &str, instance : u32, data : ()) { // TODO make data useful
	panic!("TODO")
}

pub fn exec(instancepath : &str, proofspath : &str) {
	let instancepath = Path::new(instancepath);
	let proofspath = Path::new(proofspath);
	// TODO
}

pub fn new(instancepath : &str) {
	let instancepath = Path::new(instancepath);
	// TODO
}
