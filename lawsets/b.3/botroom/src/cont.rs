use bot::Bot;

#[derive(Clone)]
pub struct BotContainer<'a> {
	botname : &'a str,
	load_fn : &'a Fn(&str) -> Bot,
	store_fn : &'a Fn(Box<Bot>),
	gen_fn : &'a Fn() -> Bot
}

pub fn get_containers<'a>() -> Vec<BotContainer<'a>> {
	panic!("")
}

impl<'a> BotContainer<'a> {
	pub fn create<'b>(botname : &'b str, load_fn : &'b Fn(&str) -> Bot, store_fn : &'b Fn(Box<Bot>), gen_fn : &'b Fn() -> Bot) -> BotContainer<'b> {
		BotContainer { botname : botname, load_fn : load_fn, store_fn : store_fn, gen_fn : gen_fn }
	}

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
