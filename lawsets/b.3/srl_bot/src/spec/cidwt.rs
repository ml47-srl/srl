use libsrl::db::Database;
use libsrl::cell::Cell;
use libsrl::navi::{CellID, CellPath};
use cond::Condition;

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

	pub fn matches(&self, condition : &Condition) -> bool {
		panic!("TODO")
	}

	pub fn get_parent(&self) -> Option<CellIDWithTarget> {
		match &self {
			&&CellIDWithTarget::CellID(ref cell_id) => {
				match cell_id.get_parent() {
					Ok(x) => Some(CellIDWithTarget::CellID(x)),
					Err(_) => None
				}
			},
			&&CellIDWithTarget::Target(ref indices) => {
				match indices.clone().pop() {
					Some(_) => Some(CellIDWithTarget::Target(indices.clone())),
					None => None
				}
			}
		}
	}

	pub fn get_children(&self, target : &Cell) -> Vec<CellIDWithTarget> {
		match &self {
			&&CellIDWithTarget::CellID(ref cell_id) => {
				let mut current_child = match cell_id.get_child(0) {
					Ok(x) => x,
					Err(_) => return vec![]
				};
				let mut children = vec![CellIDWithTarget::CellID(current_child.clone())];
				loop {
					if let Ok(x) = current_child.clone().get_right_sibling() {
						current_child = x;
						children.push(CellIDWithTarget::CellID(current_child.clone()));
					} else {
						break;
					}
				}
				return children;
			},
			&&CellIDWithTarget::Target(ref indices) => {
				let path = match CellPath::create(target.clone(), indices.clone()) {
					Ok(x) => x,
					Err(_) => panic!("CellIDWithTarget::Target::get_children(): failed to create path -- snh")
				};
				let mut current_child = match path.get_child(0) {
					Ok(x) => x,
					Err(_) => return vec![]
				};
				let mut children = vec![CellIDWithTarget::Target(current_child.get_indices())];
				loop {
					if let Ok(x) = current_child.get_right_sibling() {
						current_child = x;
						children.push(CellIDWithTarget::Target(current_child.get_indices()));
					} else {
						break;
					}
				}
				return children;
			}
		}
	}
}
