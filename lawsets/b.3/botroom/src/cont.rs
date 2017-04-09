use bot::{Bot, LinBot};

#[derive(Clone)]
pub struct BotContainer<'a> {
	botname : &'a str,
	deserialize_fn : &'a Fn(&str) -> Box<Bot>,
	serialize_fn : &'a Fn(Box<Bot>) -> String,
	gen_fn : &'a Fn() -> Box<Bot>
}

const lin_bot_cont : BotContainer<'static> = BotContainer { botname : "linbot", deserialize_fn : &|x| LinBot::by_string(x.to_string()), serialize_fn : &|x| x.to_string(), gen_fn : &|| LinBot::gen() };

pub fn get_containers<'a>() -> Vec<BotContainer<'a>> {
	vec![lin_bot_cont]
}

impl<'a> BotContainer<'a> {
	pub fn create<'b>(botname : &'b str, deserialize_fn : &'b Fn(&str) -> Box<Bot>, serialize_fn : &'b Fn(Box<Bot>) -> String, gen_fn : &'b Fn() -> Box<Bot>) -> BotContainer<'b> {
		BotContainer { botname : botname, deserialize_fn : deserialize_fn, serialize_fn : serialize_fn , gen_fn : gen_fn }
	}

	pub fn get_botname(&'a self) -> &'a str {
		self.botname
	}

	pub fn get_deserialize_fn(&'a self) -> &'a Fn(&str) -> Box<Bot> {
		&self.deserialize_fn
	}

	pub fn get_serialize_fn(&'a self) -> &'a Fn(Box<Bot>) -> String {
		&self.serialize_fn
	}

	pub fn get_gen_fn(&'a self) -> &'a Fn() -> Box<Bot> {
		&self.gen_fn
	}
}
