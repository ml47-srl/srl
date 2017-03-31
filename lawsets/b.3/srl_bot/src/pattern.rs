use libsrl::navi::CellPath;
use rand::{Rng, thread_rng};

pub enum Pattern {
	AnyCell,
	Complex { subpatterns : Vec<Pattern> },
	AnySimple,
	AnyConstant,
	BoolConstant { b : bool },
	AnyScope,
	Scope { id : usize, body : Box<Pattern> }, // the ids are not literal, so {0 } matches the pattern "{3 }"
	AnyVar,
	Var { id : usize }
}

impl Pattern {
	pub fn gen() -> Pattern {
		Pattern::gen_with_id(0)
	}

	fn gen_with_id(next_id : usize) -> Pattern {
		let mut rng = thread_rng();
		match rng.gen_range(0, 9) {
			0 => Pattern::AnyCell,
			1 => {
				let mut vec = vec![];
				for _ in 0..rng.gen_range(0, 4) {
					vec.push(Pattern::gen_with_id(next_id));
				}
				Pattern::Complex { subpatterns : vec }
			},
			2 => Pattern::AnySimple,
			3 => Pattern::AnyConstant,
			4 => {
				let b = rng.gen_range(0, 2) == 0;
				Pattern::BoolConstant { b : b }
			},
			5 => Pattern::AnyScope,
			6 => Pattern::Scope { id : next_id, body : Box::new(Pattern::gen_with_id(next_id + 1)) },
			7 => Pattern::AnyVar,
			8 => {
				if next_id == 0 {
					panic!("well... this will chance should depend on how many scopes are there")
				}
				Pattern::Var { id : rng.gen_range(0, next_id) }
			},
			_ => panic!("Pattern::gen_with_id: outta range -- snh")
		}
	}

	pub fn matched_by(&self, c : &CellPath) -> bool {
		panic!("TODO")
	}
}
