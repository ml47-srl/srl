use libsrl::cell::Cell;
use libsrl::db::Database;
use libsrl::error::SRLError;

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

	pub fn to_string(&self) -> String {
		panic!("TODO")
	}

	pub fn from_string(string : String) -> Result<Proof, SRLError> {
		panic!("TODO")
	}
}
