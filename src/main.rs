extern crate irc;

use std::default::Default;
use irc::client::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let cfg = Config {
        nickname: Some(format!("rocketlobot")),
        server: Some(format!("irc.chat.twitch.tv")),
        port: Some(6667),
        channels: Some(vec![format!("#rocketlobster")]),
        password: Some(format!("oauth:mqn0sjxtuojbstj81r37h3dztixk9c")),
        .. Default::default()
    };

    let server = IrcServer::from_config(cfg).unwrap();
    server.identify().unwrap();
    let server2 = server.clone();

    thread::spawn(move || {
        server2.iter().map(|m| print!("{}", m.unwrap())).count();
    });
    loop {
        server.send_privmsg("#rocketlobster", "This is a message").unwrap();
        thread::sleep(Duration::new(10, 0));
    }
    // for message in server.iter() {
    //     let message = message.unwrap(); //Just panic
    //     println!("{}", message);
    //     match message.command {
    //         Command::PRIVMSG(ref target, ref msg) => if msg.contains("pickles") {
    //             server.send_privmsg(target, "Hi!").unwrap();
    //         },
    //         _ => (),
    //     }
    // }
    
}
