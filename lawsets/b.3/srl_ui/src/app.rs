extern crate ncurses;
use libsrl::error::SRLError;
use libsrl::db::Database;
use libsrl::navi::CellID;

pub struct App {
	pub db : Database,
	pub prim_marker : CellID,
	pub sec_markers : Vec<CellID>,
}

impl App {
	pub fn by_filename(string : &str) -> Result<App, SRLError> {
		let db = Database::by_filename(string)?;
		let cell_id = CellID::create(0, vec![]);
		Ok(App { db : db, prim_marker :  cell_id, sec_markers : Vec::new()})
	}

	pub fn run(&mut self) {
		ncurses::initscr();
		ncurses::start_color();
		ncurses::init_pair(1, ncurses::COLOR_WHITE, ncurses::COLOR_BLACK); // default
		ncurses::init_pair(2, ncurses::COLOR_BLUE, ncurses::COLOR_BLACK); // primary marker
		ncurses::init_pair(3, ncurses::COLOR_GREEN, ncurses::COLOR_BLACK); // secondary markers

		self.render();
		while self.handle_key(ncurses::getch()) {
			self.render();
		}
		ncurses::endwin();
	}
}
