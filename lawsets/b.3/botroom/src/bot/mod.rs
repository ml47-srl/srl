mod linbot;
pub use self::linbot::LinBot;
use libsrl::db::Database;
use libsrl::cell::Cell;

pub trait Bot {
	fn to_string(&self) -> String;
	fn practice(&mut self, &Cell, &mut Database) -> bool;
	fn proof(&self, &Cell, &mut Database) -> bool;
}
