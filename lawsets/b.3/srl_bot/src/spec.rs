use pattern::Pattern;
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::cell::Cell;

enum Starter { ALL, CORE, TARGET }

pub struct Spec {
	steps : Vec<SpecStep>,
	starter : Starter
}

enum SpecStep {
	Which(Pattern),
	Parent,
	ParentR,
	parentRE,
	Child(Pattern),
	ChildR(Pattern),
	ChildRE(Pattern)
}

impl Spec {
	pub fn gen() -> Spec {
		panic!("return random Spec")
	}

	pub fn get_cell_ids(&self, db : &mut Database) -> Vec<CellID> {
		let mut vec;
		match self.starter {
			Starter::ALL => {
				vec = vec![];
				for i in 0..db.count_rules() {
					vec.push(CellID::create(i, vec![]));
				}
			},
			Starter::CORE => {
				vec = vec![CellID::create(0, vec![])];
			},
			Starter::TARGET => {
				vec = vec![];
			}
		}
		vec
	}

	pub fn get_cells(&self, db : &mut Database, target : Cell) -> Vec<Cell> {
		return match self.starter {
			Starter::ALL => {
				db.get_rules()
			},
			Starter::CORE => {
				vec![db.get_rule(0)]
			},
			Starter::TARGET => {
				vec![target]
			}
		};
	}

	pub fn get_indices(&self, db : &mut Database) -> Vec<Vec<usize>> {
		panic!("")
	}
}
