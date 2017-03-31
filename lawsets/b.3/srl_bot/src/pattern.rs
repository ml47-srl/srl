use libsrl::cell::{CellType, Cell};
use libsrl::gen::simple_by_str;
use rand::{Rng, thread_rng};

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Pattern {
	AnyCell,
	Complex { subpatterns : Vec<Pattern> },
	AnySimple,
	AnyConstant,
	BoolConstant { b : bool },
	AnyScope,
	Scope { id : usize, body : Box<Pattern> },
	AnyVar,
	Var { id : usize }
}

impl Pattern {
	pub fn gen() -> Pattern {
		Pattern::gen_with_id(0)
	}

	fn gen_with_id(next_id : usize) -> Pattern {
		let mut rng = thread_rng();
		let max_chance = if next_id == 0 { 8 } else { 9 };
		match rng.gen_range(0, max_chance) {
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
					panic!("Pattern::gen_with_id: trying to generate var without scope around")
				}
				Pattern::Var { id : rng.gen_range(0, next_id) }
			},
			_ => panic!("Pattern::gen_with_id: outta range -- snh")
		}
	}

	pub fn matched_by(&self, c : &Cell) -> bool {
		match &self {
			&&Pattern::AnyCell => true,
			&&Pattern::Complex { subpatterns : ref subpatterns_out } => {
				if let &Cell::Complex { cells : ref cells_out } = c {
					if subpatterns_out.len() != cells_out.len() {
						return false;
					}

					for i in 0..subpatterns_out.len() {
						if !subpatterns_out[i].matched_by(&cells_out[i]) { return false; }
					}
					return true;
				} else {
					return false;
				}
			},
			&&Pattern::AnySimple => c.get_type() == CellType::Simple,
			&&Pattern::AnyConstant => c.is_constant(),
			&&Pattern::BoolConstant { b : b_out } => {
				let my_cell = if b_out {
					simple_by_str("'true'")
				} else {
					simple_by_str("'false'")
				};
				&my_cell == c
			},
			&&Pattern::AnyScope => c.get_type() == CellType::Scope,
			&&Pattern::Scope { id : id_out, body : ref body_out } => {
				if let &Cell::Scope { id : id_out2, body : ref body_out2 } = c {
					if id_out != id_out2 as usize { return false; }
					body_out.matched_by(body_out2.as_ref())
				} else { return false; }
			},
			&&Pattern::AnyVar => c.get_type() == CellType::Var,
			&&Pattern::Var { id : id_out } => {
				if let &Cell::Var { id : id_out2 } = c {
					id_out == id_out2 as usize
				} else { return false; }
			}
		}
	}
}
