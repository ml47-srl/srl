mod step;
mod cidwt;

use self::step::SpecStep;
use self::cidwt::CellIDWithTarget;
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::cell::Cell;

enum Starter { ALL, CORE, TARGET }

pub struct Spec {
	steps : Vec<SpecStep>,
	starter : Starter
}

impl Spec {
	pub fn gen() -> Spec {
		panic!("return random Spec")
	}

	fn get_cell_idwts(&self, db : &mut Database, target : &Cell) -> Vec<CellIDWithTarget> {
		let mut vec;
		match self.starter {
			Starter::ALL => {
				vec = vec![];
				for i in 0..db.count_rules() {
					vec.push(CellIDWithTarget::CellID(CellID::create(i, vec![])));
				}
			},
			Starter::CORE => {
				vec = vec![CellIDWithTarget::CellID(CellID::create(0, vec![]))];
			},
			Starter::TARGET => {
				vec = vec![CellIDWithTarget::Target(vec![])];
			}
		}

		for step in &self.steps {
			vec = step.translate(vec, target);
		}
		return vec;
	}

	pub fn get_cell_ids(&self, db : &mut Database, target : &Cell) -> Vec<CellID> {
		let mut vec = vec![];
		for c in self.get_cell_idwts(db, target) {
			match c {
				CellIDWithTarget::CellID(cell_id) => { vec.push(cell_id); },
				CellIDWithTarget::Target(_) => {}
			}
		}
		vec
	}

	pub fn get_cells(&self, db : &mut Database, target : &Cell) -> Vec<Cell> {
		let mut vec = vec![];
		for c in self.get_cell_idwts(db, target) {
			vec.push(c.get_cell(db, target));
		}
		vec
	}

	pub fn get_indices(&self, db : &mut Database, target : &Cell) -> Vec<Vec<usize>> {
		let mut vec = vec![];
		for c in self.get_cell_idwts(db, target) {
			match c {
				CellIDWithTarget::CellID(cell_id) => { vec.push(cell_id.get_indices()); },
				CellIDWithTarget::Target(x) => { vec.push(x); }
			}
		}
		vec
	}
}
