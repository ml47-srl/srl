use super::cidwt::CellIDWithTarget;
use pattern::Pattern;

pub enum SpecStep {
	Which(Pattern),
	Parent,
	ParentR,
	ParentRE,
	Child(Pattern),
	ChildR(Pattern),
	ChildRE(Pattern)
}

impl SpecStep {
	pub fn translate(&self, vec : Vec<CellIDWithTarget>) -> Vec<CellIDWithTarget> {
		let mut new_vec = vec![];
		for x in vec {
			for y in self.translate_single(x) {
				new_vec.push(y);
			}
		}
		new_vec
	}

	fn translate_single(&self, c : CellIDWithTarget) -> Vec<CellIDWithTarget> {
		match &self {
			&&SpecStep::Which(ref pattern) => {
				if c.matches(pattern) {
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
			&&SpecStep::Child(ref pattern) => {
				let mut vec = vec![];
				for child in c.get_children() {
					if child.matches(pattern) {
						vec.push(child);
					}
				}
				return vec;
			},
			&&SpecStep::ChildR(ref pattern) => {
				let mut vec = vec![c];
				let mut done = false;
				while !done {
					done = true;
					for v in vec.clone() {
						for child in v.get_children() {
							if child.matches(&pattern) && !vec.contains(&child) {
								done = false;
								vec.push(child);
							}
						}
					}
				}
				return vec;
			},
			&&SpecStep::ChildRE(ref pattern) => {
				let mut vec = vec![c];
				let mut done = false;
				while !done {
					done = true;
					for v in vec.clone() {
						for child in v.get_children() {
							if child.matches(&pattern) && !vec.contains(&child) {
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
}

