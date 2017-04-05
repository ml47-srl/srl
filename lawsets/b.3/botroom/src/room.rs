use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Write, BufWriter};

use serde_json;

use fs::{assert_dir, ls};
use bot::Bot;
use cont::{BotContainer, get_containers};
use proof::Proof;

pub struct Room<'a> {
	path : &'a Path
}

impl<'a> Room<'a> {

	// loads bots & proofs from file or creates an empty folder for it
	pub fn init(path_str : &'a str) -> Room<'a> {
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

	fn get_free_res_file(&self, botname : &str, instance : u32) -> PathBuf {
		panic!("TODO")
	}

	fn get_path(&self) -> &Path {
		&self.path
	}

	fn get_container_by_botname(&self, botname : &str) -> Option<BotContainer> {
		for c in get_containers() {
			if c.get_botname() == botname {
				return Some(c.clone());
			}
		}
		None
	}

	pub fn add_bot(&mut self, bot : Box<Bot>) {
		panic!("TODO")
	}

	fn get_existing_botnames(&self) -> Vec<String> {
		ls(self.get_bots_path().as_path())
	}

	fn count_botname_instances(&self, botname : &str) -> u32 {
		ls(self.get_bots_path().join(botname).as_path()).len() as u32
	}

	fn count_botinstance_work(&self, botname : &str, i : u32) -> u32 {
		let path_buf : PathBuf = self.get_bots_path().join(botname).join(&i.to_string());
		ls(path_buf.as_path()).len() as u32
	}

	fn get_botname_with_least_work(&self) -> String {
		let mut smallest_work = None;
		let mut smallest_botname = None;

		for botname in self.get_existing_botnames() {
			let mut work = 0;
			for i in 0..self.count_botname_instances(&botname) {
				work += self.count_botinstance_work(&botname, i);
			}
			if smallest_work == None || smallest_work.unwrap() > work {
				smallest_work = Some(work);
				smallest_botname = Some(botname);
			}
		}
		smallest_botname.unwrap()
	}

	fn get_smallest_instance_for_botname(&self, botname : &str) -> u32 {
		let mut smallest_work = None;
		let mut smallest_instance = None;

		for i in 0..self.count_botname_instances(botname) {
			let work = self.count_botinstance_work(botname, i);
			if smallest_work == None || smallest_work.unwrap() > work {
				smallest_work = Some(work);
				smallest_instance = Some(i);
			}
		}
		smallest_instance.unwrap() as u32
	}

	fn get_proofs(&self) -> Vec<Proof> {
		panic!("TODO")
	}

	fn add_res(&self, botname : &str, instance : u32, data : ()) {
		let res_file = self.get_free_res_file(botname, instance);
		let mut log_file = File::create(res_file).unwrap();
		log_file.write("ok".as_bytes());
	}

	pub fn tick(&self) {
		let botname = self.get_botname_with_least_work();
		let instance = self.get_smallest_instance_for_botname(&botname);

		let path_buf : PathBuf = self.get_bots_path().join(&botname).join(&instance.to_string()).join("botfile");
		let path : &Path = path_buf.as_path();
		let mut string = String::new();
		let mut file = File::open(path).unwrap();
		file.read_to_string(&mut string);

		let bot : &mut Bot = &mut self.get_container_by_botname(&botname).unwrap().get_load_fn()(&string);

		let mut practice_vec = Vec::new();
		for proof in self.get_proofs() {
			practice_vec.push(bot.practice(&proof.get_target(), &mut proof.get_db()));
		}

		let mut proof_vec = Vec::new();
		for proof in self.get_proofs() {
			proof_vec.push(bot.proof(&proof.get_target(), &mut proof.get_db()));
		}
		file.write(bot.to_string().as_bytes());
		self.add_res(&botname, instance, ());
	}
}
