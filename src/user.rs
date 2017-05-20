use irc::client::prelude::*;

pub enum Badge {
	Broadcaster,
	Moderator,
	Subscriber,
	Prime,
	Turbo
}

pub struct User {
	pub display_name: String,
	pub badges: Vec<Badge>,

}

impl User {
	pub fn new(m: &Message) -> User {
		let mut name = String::new();
		for tag in m.tags.as_ref().unwrap()	 {
			println!("{:#?}", tag);
			if tag.0.as_str() == "display-name" {
				name = tag.1.clone().unwrap();
			}
		}
		let badges: Vec<Badge> = Vec::new();

		User {
			display_name: name,
			badges: badges
		}
	}
}