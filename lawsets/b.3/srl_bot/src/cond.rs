use pattern::Pattern;
use rand::{Rng, thread_rng};
use libsrl::navi::CellPath;

pub enum Condition {
	Pattern(Pattern),
	Bool,
	CompleteBool,
	Wrapper,
	NexqWrapper,
	NallqWrapper,
	PositiveWrapper,
	NegativeWrapper,
}

impl Condition {
	pub fn gen() -> Condition {
		let mut rng = thread_rng();
		return match rng.gen_range(0, 8) {
			0 => Condition::Pattern(Pattern::gen()),
			1 => Condition::Bool,
			2 => Condition::CompleteBool,
			3 => Condition::Wrapper,
			4 => Condition::NexqWrapper,
			5 => Condition::NallqWrapper,
			6 => Condition::PositiveWrapper,
			7 => Condition::NegativeWrapper,
			_ => panic!("Condition::gen() outta range -- snh")
		};
	}

	pub fn matched_by(&self, c : &CellPath) -> bool {
		panic!("TODO")
	}
}
