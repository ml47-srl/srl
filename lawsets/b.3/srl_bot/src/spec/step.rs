use super::CellIDWithTarget;
use pattern::Pattern;

pub enum SpecStep {
	Which(Pattern),
	Parent,
	ParentR,
	parentRE,
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
		panic!("")
	}
}

