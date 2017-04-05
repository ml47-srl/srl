use bot::Bot;
use proof::Proof;
use std::path::{Path, PathBuf};
use fs::assert_dir;

use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use serde_json;

pub struct Room<'a> {
	path : &'a Path
}

impl<'a> Room<'a> {

	// loads bots & proofs from file or creates an empty folder for it
	pub fn init(path_str : &'a str) -> Room {
		let room = Room { path : Path::new(path_str) };
		assert_dir(room.get_path());
		assert_dir(room.get_bots_path().as_path());
		assert_dir(room.get_proofs_path().as_path());
		room
	}

	fn get_bots_path(&self) -> PathBuf {
		self.path.join("bots")
	}

	fn get_proofs_path(&self) -> PathBuf {
		self.path.join("proofs")
	}

	pub fn add_proof(&mut self, proof : Proof) {
		let file = File::open(self.get_free_proof_file()).expect("Room::add_proof: failed opening get_free_proof_file()");
		let string = serde_json::to_string(&proof).expect("Room::add_proof: failed serializing proof");
		let mut bw = BufWriter::new(file);
		bw.write(string.as_bytes());
	}

	fn get_free_proof_file(&self) -> PathBuf {
		let mut i = 0;
		while self.get_path().join("p".to_string() + &i.to_string()).exists() {
			i += 1;
		}
		self.get_path().join("p".to_string() + &i.to_string())
	}

	fn get_path(&self) -> &Path {
		&self.path
	}

	pub fn add_bot(&mut self, bot : Box<Bot>) {
		panic!("TODO")
	}

	pub fn tick(&self) {
		panic!("TODO")
	}
}
