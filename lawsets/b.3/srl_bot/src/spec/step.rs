use super::cidwt::CellIDWithTarget;
use libsrl::cell::Cell;
use rand::{Rng, thread_rng};
use cond::Condition;

pub enum SpecStep {
	Which(Condition),
	Parent,
	ParentR,
	ParentRE,
	Child(Condition),
	ChildR(Condition),
	ChildRE(Condition)
}

impl SpecStep {
	pub fn translate(&self, vec : Vec<CellIDWithTarget>, target : &Cell) -> Vec<CellIDWithTarget> {
		let mut new_vec = vec![];
		for x in vec {
			for y in self.translate_single(x, target) {
				new_vec.push(y);
			}
		}
		new_vec
	}

	fn translate_single(&self, c : CellIDWithTarget, target : &Cell) -> Vec<CellIDWithTarget> {
		match &self {
			&&SpecStep::Which(ref cond) => {
				if c.matches(cond) {
					return vec![c];
				} else {
					return vec![];
				}
			},
			&&SpecStep::Parent => {
				match c.get_parent() {
					Some(x) => return vec![x],
					None => return vec![]
				}
			},
			&&SpecStep::ParentR => {
				let mut vec = vec![c.clone()];
				let mut newest = c;
				loop {
					match newest.get_parent() {
						Some(x) => { vec.push(x.clone()); newest = x; }
						None => return vec
					}
				}
			},
			&&SpecStep::ParentRE => {
				let mut c_new = c;
				loop {
					match c_new.get_parent() {
						Some(x) => c_new = x,
						None => return vec![c_new]
					}
				}
			},
			&&SpecStep::Child(ref cond) => {
				let mut vec = vec![];
				for child in c.get_children(target) {
					if child.matches(cond) {
						vec.push(child);
					}
				}
				return vec;
			},
			&&SpecStep::ChildR(ref cond) => {
				let mut vec = vec![c];
				let mut done = false;
				while !done {
					done = true;
					for v in vec.clone() {
						for child in v.get_children(target) {
							if child.matches(&cond) && !vec.contains(&child) {
								done = false;
								vec.push(child);
							}
						}
					}
				}
				return vec;
			},
			&&SpecStep::ChildRE(ref cond) => {
				let mut vec = vec![c];
				let mut done = false;
				while !done {
					done = true;
					for v in vec.clone() {
						for child in v.get_children(target) {
							if child.matches(&cond) && !vec.contains(&child) {
								done = false;
								vec.push(child);
								match vec.iter().position(|ref a| **a == v) {
									Some(x) => { vec.remove(x); }
									None => {}
								}
							}
						}
					}
				}
				return vec;
			},
		}
	}

	pub fn gen() -> SpecStep {
		let mut rng = thread_rng();
		return match rng.gen_range(0, 7) {
			0 => SpecStep::Which(Condition::gen()),
			1 => SpecStep::Parent,
			2 => SpecStep::ParentR,
			3 => SpecStep::ParentRE,
			4 => SpecStep::Child(Condition::gen()),
			5 => SpecStep::ChildR(Condition::gen()),
			6 => SpecStep::ChildRE(Condition::gen()),
			_ => panic!("SpecStep::gen() outta range -- snh")
		};
	}
}
