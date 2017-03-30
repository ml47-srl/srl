mod step;

use self::step::SpecStep;
use pattern::Pattern;
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::navi::CellPath;
use libsrl::cell::Cell;

enum Starter { ALL, CORE, TARGET }

pub struct Spec {
	steps : Vec<SpecStep>,
	starter : Starter
}

enum CellIDWithTarget {
	CellID(CellID),
	Target(Vec<usize>)
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
			vec = step.translate(vec);
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

impl CellIDWithTarget {
	fn get_cell(&self, db : &mut Database, target : &Cell) -> Cell {
		let cell_path = match &self {
			&&CellIDWithTarget::CellID(ref cell_id) => { cell_id.get_path(&db.get_rules()) },
			&&CellIDWithTarget::Target(ref x) => { CellPath::create(target.clone(), x.clone()) }
		};
		match cell_path {
			Ok(x) => return x.get_cell(),
			Err(_) => panic!("CellIDWithTarget err")
		}
	}
}
