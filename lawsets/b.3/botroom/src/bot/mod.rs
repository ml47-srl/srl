mod linbot;
pub use self::linbot::LinBot;
use libsrl::db::Database;
use libsrl::error::SRLError;
use libsrl::cell::Cell;

pub trait Bot {
	fn to_file(&self, &str) -> Result<(), SRLError>;
	fn practice(&mut self, &Cell, &mut Database) -> bool;
	fn proof(&self, &Cell, &mut Database) -> bool;
}
