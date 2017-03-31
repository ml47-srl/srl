use idea::Idea;
use libsrl::cell::Cell;
use libsrl::db::Database;
use rand::{Rng, thread_rng};

pub struct Bot {
	ideas : Vec<WeightedIdea>
}

struct WeightedIdea {
	idea : Idea,
	niceness : i32,
	familiarness : u32 // number of usages
}

impl Bot {
	pub fn proof(&mut self, rule : &Cell, db : &mut Database) -> bool {
		for _ in 0..20 {
			let index = self.get_next_idea_index();
			if self.ideas[index].proof(rule, db) {
				self.ideas[index].eval(1);

				if self.ideas[index].get_weighted_niceness() > 100 {
					let mutation = self.ideas[index].mutate();
					self.ideas.push(mutation); // XXX would cause too many mutations sometimes
				}
				return true;
			} else {
				self.ideas[index].eval(-1);

				if self.ideas[index].get_weighted_niceness() < -100 {
					self.ideas.remove(index);
				}
			}
		}
		false
	}

	pub fn gen() -> Bot {
		let mut ideas = vec![];
		for _ in 0..40 {
			ideas.push(WeightedIdea::gen())
		}
		Bot { ideas : ideas }
	}

	fn get_next_idea_index(&self) -> usize {
		let mut rng = thread_rng();
		rng.gen_range(0, self.ideas.len())
	}
}

impl WeightedIdea {
	fn gen() -> WeightedIdea {
		WeightedIdea { idea : Idea::gen(), niceness : 0, familiarness : 0 }
	}

	fn get_weighted_niceness(&self) -> i32 {
		self.niceness * self.familiarness as i32 // XXX 10 fails & 1 win (-9 * 11) should be better than 9 fails (-9 * 9)
	}

	fn eval(&mut self, evaluation : i32) {
		self.familiarness += 1;
		self.niceness += evaluation;
	}

	fn mutate(&self) -> WeightedIdea {
		let keep = self.get_weighted_niceness();
		WeightedIdea { idea : self.idea.mutate(keep), niceness : 0, familiarness : 0 }
	}

	fn proof(&self, rule : &Cell, db : &mut Database) -> bool {
		self.idea.proof(rule, db)
	}
}
