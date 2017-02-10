pub mod libsrl {
	mod misc {
		pub fn do_miscellaneous_stuff() {
			println!("stuff..");
		}
	}

	pub mod error {
		pub type SRLError = String;
	}

	mod parse {
		use libsrl::SimpleCell;
		use libsrl::error::SRLError;

		mod assemble {
			use libsrl::SimpleCell;
			use libsrl::error::SRLError;

			fn assemble_detail() { println!("assemble_detail running!"); }
			pub fn assemble(vec : Vec<String>) -> Result<SimpleCell, SRLError> {
				assemble_detail();
				super::super::misc::do_miscellaneous_stuff();
				return Ok(SimpleCell { string : vec[0].clone() });
			}
		}

		mod tokenize {
			use libsrl::error::SRLError;

			fn tokenize_detail() { println!("detailed parsing processing... is processing"); }
			pub fn tokenize(s : String) -> Result<Vec<String>, SRLError> {
				tokenize_detail();
				return Ok(vec!["this is tokenized code".to_string()]);
			}
		}

		pub fn parse(s : String) -> Result<SimpleCell, SRLError> {
			match tokenize::tokenize(s) {
				Ok(tokens) => {
					match assemble::assemble(tokens) {
						Ok(cell) => return Ok(cell),
						Err(srl_error) => return Err(srl_error)
					}
				}
				Err(srl_error) => return Err(srl_error)
			}
		}
	}

	pub struct SimpleCell { string : String }

	pub mod db {
		use libsrl::SimpleCell;
		use libsrl::error::SRLError;

		pub struct Db {
			rules : Vec<SimpleCell>
		}

		impl Db {
			pub fn db_get() -> Result<Db, SRLError> {
				match super::parse::parse("test".to_string()) {
					Ok(cell) => Ok(Db { rules : vec![cell] }),
					Err(srl_error) => return Err(srl_error)
				}
			}
		}
	}
}

fn main() {
	use libsrl::db;
	let d = db::Db::db_get();
}
