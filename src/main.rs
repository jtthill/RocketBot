#[macro_use]
extern crate log;
extern crate fern;
extern crate irc;
extern crate time;

pub mod config;
pub mod twitch;
pub mod logging;

use irc::client::prelude::*;
use std::default::Default;

pub fn main() {

    // Set up logging
    if let Err(e) = logging::setup() {
        panic!("Failed to initialize logging: {}", e);
    }
    debug!("Begin logging.");

    let cfg = config::init_configs();
    let bot = twitch::TwitchBot::new(cfg);
    bot.run();
}
