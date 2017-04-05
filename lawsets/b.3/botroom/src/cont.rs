use bot::Bot;

#[derive(Clone)]
pub struct BotContainer<'a> {
	load_fn : &'a Fn(&str) -> Bot,
	store_fn : &'a Fn(Box<Bot>),
	gen_fn : &'a Fn() -> Bot,
	botname : &'a str
}

impl<'a> BotContainer<'a> {
	pub fn get_botname(&'a self) -> &'a str {
		self.botname
	}

	pub fn get_load_fn(&'a self) -> &'a Fn(&str) -> Bot {
		&self.load_fn
	}

	pub fn get_store_fn(&'a self) -> &'a Fn(Box<Bot>) {
		&self.store_fn
	}

	pub fn get_gen_fn(&'a self) -> &'a Fn() -> Bot {
		&self.gen_fn
	}
}
