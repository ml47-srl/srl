use libsrl::cell::Cell;
use libsrl::db::Database;

#[derive(Serialize, Deserialize)]
pub struct Proof {
	target_string : String,
	db_string : String
}

impl Proof {
	pub fn create(target_string : String, db_string : String) -> Proof {
		Proof { target_string : target_string, db_string : db_string }
	}

	pub fn get_target(&self) -> Cell {
		Cell::by_string(&self.target_string).unwrap()
	}

	pub fn get_db(&self) -> Database {
		Database::by_string(&self.db_string).unwrap()
	}
}
