use action::Action;
use libsrl::cell::Cell;
use libsrl::db::Database;

pub struct Solver {
	actions : Vec<Action>
}

impl Solver {
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
				db.delete_rule(i);
			}
		}
		result_i.is_some()
	}

	pub fn gen() -> Solver {
		panic!("generate random Solver")
	}
}
