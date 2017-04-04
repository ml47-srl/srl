use bot::Bot;
use proof::Proof;

pub struct Room {
	bots : Vec<Box<Bot>>,
	proofs : Vec<Proof>
}

impl Room {
	pub fn new() -> Room {
		Room { bots : vec![], proofs : vec![] }
	}

	pub fn add_proof(&mut self, proof : Proof) {
		self.proofs.push(proof);
	}

	pub fn add_bot(&mut self, bot : Box<Bot>) {
		self.bots.push(bot);
	}

	pub fn run(&mut self) {
		// TODO
	}
}
