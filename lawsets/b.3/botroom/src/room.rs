use std::path::{Path, PathBuf};

use serde_json;

use fs::*;
use bot::Bot;
use cont::{BotContainer, get_containers};
use proof::Proof;
use libsrl::error::SRLError;

pub struct Room<'a> {
	path : &'a Path
}
/*
	The complete error-system asserts that there are the directories:
		room.get_path(),
		room.get_bots_pbuf().as_path(),
		room.get_proofs_pbuf().as_path()
	exist

	The existance of all other directories has to be checked
*/

impl<'a> Room<'a> {

	// loads bots & proofs from file or creates an empty folder for it
	pub fn init(path_str : &'a str) -> Room<'a> {
		let room = Room { path : Path::new(path_str) };
		assert_dir(room.get_path());
		assert_dir(room.get_bots_pbuf().as_path());
		assert_dir(room.get_proofs_pbuf().as_path());
		room
	}

	fn get_bots_pbuf(&self) -> PathBuf {
		self.path.join("bots")
	}

	fn get_bot_pbuf(&self, botname : &str) -> PathBuf {
		self.get_bots_pbuf().join(botname)
	}


	fn get_botinstance_pbuf(&self, botname : &str, instance : u32) -> PathBuf {
		self.get_bot_pbuf(botname).join(&instance.to_string())
	}

	fn get_botinstance_res_pbuf(&self, botname : &str, instance : u32, res_id : u32) -> PathBuf {
		self.get_botinstance_pbuf(botname, instance).join(&res_id.to_string())
	}

	fn get_proofs_pbuf(&self) -> PathBuf {
		self.path.join("proofs")
	}

	fn get_proof_pbuf(&self, proof_id : u32) -> PathBuf {
		self.get_proofs_pbuf().join(&proof_id.to_string())
	}

	pub fn add_proof(&mut self, proof : Proof) -> Result<(), SRLError> {
		let pbuf = self.get_free_proof_pbuf();
		let string = match serde_json::to_string(&proof) {
			Ok(x) => x,
			Err(_) => return Err(SRLError("add_proof".to_string(), "serialization failed".to_string()))
		};
		write_file(pbuf.as_path(), &string)?;
		Ok(())
	}

	fn get_free_proof_pbuf(&self) -> PathBuf {
		let mut i = 0;
		while self.get_proof_pbuf(i).exists() {
			i += 1;
		}
		self.get_proof_pbuf(i)
	}

	fn get_free_res_pbuf(&self, botname : &str, instance : u32) -> PathBuf {
		let mut i = 0;
		while self.get_botinstance_res_pbuf(botname, instance, i).exists() {
			i += 1;
		}
		self.get_botinstance_res_pbuf(botname, instance, i)
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

	fn get_botnames(&self) -> Vec<String> {
		match ls(self.get_bots_pbuf().as_path()) {
			Ok(x) => x,
			Err(_) => panic!("get_botnames failed -- snh")
		}
	}


	fn count_botname_instances(&self, botname : &str) -> Result<u32, SRLError> {
		Ok(ls(self.get_bots_pbuf().join(botname).as_path())?.len() as u32)
	}

	fn count_botinstance_work(&self, botname : &str, instance : u32) -> Result<u32, SRLError> {
		let pbuf : PathBuf = self.get_botinstance_pbuf(botname, instance);
		Ok(ls(pbuf.as_path())?.len() as u32)
	}

	fn get_botname_with_least_work(&self) -> Result<String, SRLError> {
		let mut smallest_work = None;
		let mut smallest_botname = None;

		for botname in self.get_botnames() {
			let mut work = 0;
			for i in 0..self.count_botname_instances(&botname)? {
				work += self.count_botinstance_work(&botname, i)?;
			}
			if match smallest_work {
				Some(x) => x > work,
				None => true
			}
			{
				smallest_work = Some(work);
				smallest_botname = Some(botname);
			}
		}
		match smallest_botname {
			Some(x) => Ok(x),
			None => Err(SRLError("get_botname_with_least_work".to_string(), "no botname found".to_string()))
		}
	}

	fn get_smallest_instance_for_botname(&self, botname : &str) -> Result<u32, SRLError> {
		let mut smallest_work = None;
		let mut smallest_instance = None;

		for i in 0..self.count_botname_instances(botname)? {
			let work = self.count_botinstance_work(botname, i)?;
			if match smallest_work {
				Some(x) => x > work,
				None => true
			}
			{
				smallest_work = Some(work);
				smallest_instance = Some(i);
			}
		}
		match smallest_instance {
			Some(x) => Ok(x),
			None => Err(SRLError("get_smallest_instance_for_botname".to_string(), "no instance found".to_string()))
		}
	}

	fn get_proofs(&self) -> Vec<Proof> {
		let mut i = 0;
		let mut vec = Vec::new();
		while self.get_proof_pbuf(i).exists() {
			let pbuf = self.get_proof_pbuf(i);
			let string = read_file(pbuf.as_path()).unwrap();
			let proof = match serde_json::from_str(&string) {
				Ok(x) => x,
				Err(_) => panic!("failed loading proof from file")
			};
			vec.push(proof);
			i += 1;
		}
		vec
	}

	fn add_res(&self, botname : &str, instance : u32, data : ()) { // TODO make data useful
		let pbuf = self.get_free_res_pbuf(botname, instance);
		write_file(pbuf.as_path(), "ok").unwrap();
	}

	pub fn tick(&self) {
		let botname = self.get_botname_with_least_work().unwrap();
		let instance = self.get_smallest_instance_for_botname(&botname).unwrap();

		let path_buf : PathBuf = self.get_bots_pbuf().join(&botname).join(&instance.to_string()).join("botfile");
		let path : &Path = path_buf.as_path();
		let string = read_file(path).unwrap();

		let container : BotContainer = match self.get_container_by_botname(&botname) {
			Some(x) => x,
			None => panic!("invalid shit")
		};
		let load_fn = container.get_load_fn();
		let bot : &mut Bot = &mut *load_fn(&string);

		let mut practice_vec = Vec::new();
		for proof in self.get_proofs() {
			practice_vec.push(bot.practice(&proof.get_target(), &mut proof.get_db()));
		}

		let mut proof_vec = Vec::new();
		for proof in self.get_proofs() {
			proof_vec.push(bot.proof(&proof.get_target(), &mut proof.get_db()));
		}
		write_file(path, &bot.to_string()).unwrap();
		self.add_res(&botname, instance, ());
	}
}
