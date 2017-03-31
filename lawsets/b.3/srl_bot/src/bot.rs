use idea::Idea;
use libsrl::cell::Cell;
use libsrl::db::Database;
use libsrl::error::SRLError;
use rand::{Rng, thread_rng};
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bot {
	ideas : Vec<WeightedIdea>
}

#[derive(Serialize, Deserialize, Debug)]
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

	pub fn from_file(filename : &str) -> Result<Bot, SRLError> {
		let file = match File::open(filename) {
			Ok(x) => x,
			Err(_) => return Err(SRLError("Bot::from_file".to_string(), format!("error opening file '{}'", filename)))
		};
		let mut string = String::new();
		let mut br = BufReader::new(file);
		match br.read_to_string(&mut string) {
			Ok(_) => {},
			Err(_) => return Err(SRLError("Bot::from_file".to_string(), format!("error reading file '{}'", filename)))
		}
		match serde_json::from_str(&string) {
			Ok(x) => Ok(x),
			Err(_) => Err(SRLError("Bot::from_file".to_string(), format!("serde_json::from_str failed on file '{}'", filename)))
		}
	}

	pub fn to_file(&self, filename : &str) -> Result<(), SRLError> {
		let file = match File::create(filename) {
			Ok(x) => x,
			Err(_) => return Err(SRLError("Bot::to_file".to_string(), format!("error opening file '{}'", filename)))
		};
		let mut bw = BufWriter::new(file);
		match bw.write_all(self.to_json().as_bytes()) {
			Ok(_) => Ok(()),
			Err(_) => Err(SRLError("Bot::to_file".to_string(), format!("error writing file '{}'", filename)))
		}
	}

	pub fn to_json(&self) -> String {
		serde_json::to_string(&self).expect("serde_json::to_string failed on Bot")
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
