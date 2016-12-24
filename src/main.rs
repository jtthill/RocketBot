extern crate irc;

use std::default::Default;
use irc::client::prelude::*;

fn main() {
    let cfg = Config {
        nickname: Some(format!("rocketlobster")),
        server: Some(format!("irc.chat.twitch.tv")),
        port: Some(6667),
        channels: Some(vec![format!("#rocketlobster")]),
        password: Some(format!("oauth:607q5vftfd103fdft768l78m95qr8b")),
        .. Default::default()
    };

    let server = IrcServer::from_config(cfg).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); //Just panic
        println!("{}", message);
    }
    
}
