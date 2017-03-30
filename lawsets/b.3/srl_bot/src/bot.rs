use action::Action;
use libsrl::cell::Cell;
use libsrl::db::Database;
use rand::{Rng, thread_rng};

pub struct Bot {
	actions : Vec<Action>
}

impl Bot {
	pub fn proof(&self, rule : &Cell, db : &mut Database) -> bool {
		let mut count : usize = 0;
		for action in &self.actions {
			count += action.call(rule, db) as usize;
		}
		let len = db.count_rules();
		let result_i : Option<usize> = {
			let mut result = None;
			for i in 0..len {
				if db.get_rule(i) == *rule {
					result = Some(i);
					break;
				}
			}
			result
		};
		for i in (len - count)..len {
			if Some(i) != result_i {
				match db.delete_rule(i) {
					Ok(_) => {},
					Err(srl_error) => {
						panic!("Bot::proof(): error while deleting unused rules: -- snh -- {}", srl_error)
					}
				}
			}
		}
		result_i.is_some()
	}

	pub fn gen() -> Bot {
		let mut rng = thread_rng();
		let n : u32 = rng.gen_range(4, 30);
		let mut actions = vec![];
		for _ in 0..n {
			actions.push(Action::gen());
		}
		Bot { actions : actions }
	}
}
