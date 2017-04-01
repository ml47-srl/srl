use idea::Idea;
use libsrl::cell::Cell;
use libsrl::db::Database;
use libsrl::error::SRLError;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use serde_json;

const MIN_IDEAS : usize = 200;

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
	pub fn proof(&self, rule : &Cell, db : &mut Database) -> bool {
		for i in 0..self.ideas.len() {
			if self.ideas[i].proof(rule, db) {
				return true;
			} else {
			}
		}
		false
	}

	pub fn practice(&mut self, rule : &Cell, db : &mut Database) -> bool {
		let mut worked = false;
		for i in 0..self.ideas.len() {
			if self.ideas[i].proof(rule, db) {
				self.ideas[i].eval(1);

				if self.ideas[i].get_weighted_niceness() > 100 {
					let mutation = self.ideas[i].mutate();
					self.ideas.push(mutation); // XXX would cause too many mutations sometimes
				}
				worked = true;
			} else {
				self.ideas[i].eval(-1);

				if self.ideas[i].get_weighted_niceness() < -100 {
					self.remove_idea(i);
				}
			}
		}
		worked
	}

	fn remove_idea(&mut self, i : usize) {
		self.ideas.remove(i);
		if self.ideas.len() < MIN_IDEAS {
			self.find_new_idea();
		}
	}

	fn find_new_idea(&mut self) {
		self.ideas.push(WeightedIdea::gen()); // XXX maybe use mutation of best ideas here
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
		for _ in 0..MIN_IDEAS {
			ideas.push(WeightedIdea::gen())
		}
		Bot { ideas : ideas }
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
