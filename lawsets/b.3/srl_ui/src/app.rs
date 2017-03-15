extern crate ncurses;
use libsrl::error::SRLError;
use libsrl::db::Database;

pub struct App {
	db : Database
}

impl App {
	pub fn by_filename(string : &str) -> Result<App, SRLError> {
		Ok(App { db : Database::by_filename(string)? })
	}

	pub fn run(&self) {
		ncurses::initscr();
		self.render();
		while self.handle_key(ncurses::getch()) {
			self.render();
		}
		ncurses::endwin();
	}

	// returns whether to go on
	fn handle_key(&self, key : i32) -> bool {
		if key == 'q' as i32 {
			return false;
		}
		true
	}

	fn render(&self) {
		ncurses::clear();
		for i in 0..self.db.count_rules() {
			ncurses::printw(&self.db.get_rule(i).to_rule_string());
			ncurses::addch('\n' as u64);
		}
		ncurses::refresh();
	}
}
