use libsrl::db::Database;
use libsrl::cell::Cell;
use libsrl::navi::{CellID, CellPath};
use pattern::Pattern;

#[derive(PartialEq)]
#[derive(Clone)]
pub enum CellIDWithTarget {
	CellID(CellID),
	Target(Vec<usize>)
}

impl CellIDWithTarget {
	pub fn get_cell(&self, db : &mut Database, target : &Cell) -> Cell {
		let cell_path = match &self {
			&&CellIDWithTarget::CellID(ref cell_id) => { cell_id.get_path(&db.get_rules()) },
			&&CellIDWithTarget::Target(ref x) => { CellPath::create(target.clone(), x.clone()) }
		};
		match cell_path {
			Ok(x) => return x.get_cell(),
			Err(_) => panic!("CellIDWithTarget err")
		}
	}

	pub fn matches(&self, pattern : &Pattern) -> bool {
		panic!("TODO")
	}

	pub fn get_parent(&self) -> Option<CellIDWithTarget> {
		panic!("TODO")
	}

	pub fn get_children(&self) -> Vec<CellIDWithTarget> {
		panic!("TODO")
	}
}
