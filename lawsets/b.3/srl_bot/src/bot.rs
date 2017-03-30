use action::Action;
use libsrl::cell::Cell;
use libsrl::db::Database;

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
		panic!("generate random Bot")
	}
}
