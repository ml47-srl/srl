use libsrl::cell::Cell;
use libsrl::db::Database;
use serde::{Serialize, Serializer, Deserialize, Deserializer};

pub struct Proof {
	target : Cell,
	db : Database
}

impl Proof {
	pub fn create(target : Cell, db : Database) -> Proof {
		Proof { target : target, db : db }
	}

	pub fn get_target(&self) -> &Cell {
		&self.target
	}

	pub fn get_db(&self) -> &Database {
		&self.db
	}

}

impl Serialize for Proof {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
		panic!("TODO")
	}
}

impl Deserialize for Proof {
	fn deserialize<D>(deserializer: D) -> Result<Proof, D::Error> where D: Deserializer {
		panic!("TODO")
	}
} 
