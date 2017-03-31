use pattern::Pattern;
use rand::{Rng, thread_rng};
use libsrl::navi::CellPath;

pub enum Condition {
	Pattern(Pattern),
	Bool,
	CompleteBool,
	Wrapper { positive : Trool, nallq : Trool, nexq : Trool },
}

pub enum Trool { True, Ignore, False }

impl Condition {
	pub fn gen() -> Condition {
		let mut rng = thread_rng();
		return match rng.gen_range(0, 4) {
			0 => Condition::Pattern(Pattern::gen()),
			1 => Condition::Bool,
			2 => Condition::CompleteBool,
			3 => Condition::Wrapper { positive : Trool::gen(), nallq : Trool::gen(), nexq : Trool::gen() },
			_ => panic!("Condition::gen() outta range -- snh")
		};
	}

	pub fn matched_by(&self, c : &CellPath) -> bool {
		match &self {
			&&Condition::Pattern(ref pat) => pat.matched_by(c),
			&&Condition::Bool => c.is_bool(),
			&&Condition::CompleteBool => c.is_complete_bool(),
			&&Condition::Wrapper { positive : ref positive_out, nallq : ref nallq_out, nexq : ref nexq_out } => {
				if let Some(w) = c.get_wrapper() {
					if !(positive_out.accepts(w.is_positive())) {
						return false;
					} else if !(nallq_out.accepts(w.is_nallq())) {
						return false;
					} else if !(nexq_out.accepts(w.is_nexq())) {
						return false;
					} else {
						return true;
					}
				} else {
					return false;
				}
			}
		}
	}
}

impl Trool {
	fn accepts(&self, b : bool) -> bool {
		match &self {
			&&Trool::True => b,
			&&Trool::Ignore => true,
			&&Trool::False => !b
		}
	}

	fn gen() -> Trool {
		let mut rng = thread_rng();
		return match rng.gen_range(0, 3) {
			0 => Trool::True,
			1 => Trool::Ignore,
			2 => Trool::False,
			_ => panic!("Trool::gen() outta range -- snh")
		};
		
	}
}
