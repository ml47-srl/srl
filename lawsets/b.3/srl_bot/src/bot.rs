use idea::Idea;
use libsrl::cell::Cell;
use libsrl::db::Database;
use libsrl::error::SRLError;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use serde_json;

const MIN_IDEAS : usize = 200;

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
			}
		}
		false
	}

	pub fn practice(&mut self, rule : &Cell, db : &mut Database) -> bool {
		let mut worked = false;
		for i in 0..self.ideas.len() {
			let (time, b) = self.ideas[i].proof_timed(rule, db);
			let mut evaluation = 0;
			evaluation -= ((time as f64) / (500 as f64)) as i32;
			if b {
				evaluation += 5;
				worked = true;
			} else {
				evaluation -= 1;
			}
			self.execute_idea_evaluation(i, evaluation);
		}
		worked
	}

	fn execute_idea_evaluation(&mut self, i : usize, evaluation : i32) {
		self.ideas[i].eval(evaluation);
		let weighted_niceness = self.ideas[i].get_weighted_niceness();

		if weighted_niceness > 100 {
			let mutation = self.ideas[i].mutate();
			self.ideas.push(mutation); // XXX would cause too many mutations sometimes
		} else if weighted_niceness < -100 {
			self.remove_idea(i);
		}
		
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
		Ok(Bot::by_string(string))
	}

	pub fn to_file(&self, filename : &str) -> Result<(), SRLError> {
		let file = match File::create(filename) {
			Ok(x) => x,
			Err(_) => return Err(SRLError("Bot::to_file".to_string(), format!("error opening file '{}'", filename)))
		};
		let mut bw = BufWriter::new(file);
		match bw.write_all(self.to_string().as_bytes()) {
			Ok(_) => Ok(()),
			Err(_) => Err(SRLError("Bot::to_file".to_string(), format!("error writing file '{}'", filename)))
		}
	}

	fn to_string(&self) -> String {
		let mut string_vec = vec![];
		for idea in &self.ideas {
			string_vec.push(serde_json::to_string(&idea).expect("serde_json::to_string failed on idea"));
		}
		string_vec.join("\n")
	}

	fn by_string(string : String) -> Bot {
		let mut ideas = vec![];
		for split in string.split('\n') {
			if split.is_empty() { continue; }
			ideas.push(serde_json::from_str(&split).expect("by_string failed"));
		}
		Bot { ideas : ideas }
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

	fn proof_timed(&self, rule : &Cell, db : &mut Database) -> (i64, bool) {
		self.idea.proof_timed(rule, db)
	}

	fn proof(&self, rule : &Cell, db : &mut Database) -> bool {
		self.idea.proof(rule, db)
	}
}
