mod step;
mod lpath;

use self::step::SpecStep;
use self::lpath::{LocatedCellPath, Location};
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::cell::Cell;
use rand::{Rng, thread_rng};

#[derive(Clone)]
enum Starter { ALL, CORE, TARGET }

#[derive(Clone)]
pub struct Spec {
	steps : Vec<SpecStep>,
	starter : Starter
}

impl Spec {
	pub fn gen() -> Spec {
		let mut rng = thread_rng();
		let starter = match rng.gen_range(0, 3) {
			0 => Starter::ALL,
			1 => Starter::CORE,
			2 => Starter::TARGET,
			_ => panic!("Spec::gen() rng outta range -- snh")
		};
		let mut steps = vec![];
		for _ in 0..rng.gen_range(0, 3) {
			steps.push(SpecStep::gen());
		}
		Spec { starter : starter, steps : steps }
	}

	fn get_lpaths(&self, db : &Database, target : &Cell) -> Vec<LocatedCellPath> {
		let mut vec;
		match self.starter {
			Starter::ALL => {
				vec = vec![];
				for i in 0..db.count_rules() {
					vec.push(LocatedCellPath::create_rule(db, CellID::create(i, vec![])));
				}
			},
			Starter::CORE => {
				vec = vec![LocatedCellPath::create_rule(db, CellID::create(0, vec![]))];
			},
			Starter::TARGET => {
				vec = vec![LocatedCellPath::create_target(target.clone(), vec![])];
			}
		}

		for step in &self.steps {
			vec = step.translate(vec);
		}
		vec
	}

	pub fn get_cell_ids(&self, db : &Database, target : &Cell) -> Vec<CellID> {
		let mut vec = vec![];
		for c in self.get_lpaths(db, target) {
			let rule_index = match c.get_location() {
				Location::Rule { rule_index : x } => x,
				Location::Target => continue
			};
			vec.push(CellID::create(rule_index, c.get_indices()));
		}
		vec
	}

	pub fn get_cells(&self, db : &Database, target : &Cell) -> Vec<Cell> {
		let mut vec = vec![];
		for c in self.get_lpaths(db, target) {
			vec.push(c.get_cell());
		}
		vec
	}

	pub fn get_indices(&self, db : &Database, target : &Cell) -> Vec<Vec<usize>> { // XXX probabbly not very useful
		let mut vec = vec![];
		for c in self.get_lpaths(db, target) {
			vec.push(c.get_indices());
		}
		vec
	}
}
