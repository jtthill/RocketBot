#[macro_use]
extern crate log;
extern crate fern;
extern crate irc;
extern crate time;

use std::default::Default;
use irc::client::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {

    // Set up logging
    if let Err(e) = setup_logging() {
        panic!("Failed to initialize logging: {}", e);
    }
    info!("Begin logging.");

    let cfg = irc::client::data::Config {
        nickname: Some(format!("rocketlobot")),
        server: Some(format!("irc.chat.twitch.tv")),
        port: Some(6667),
        channels: Some(vec![format!("#rocketlobster")]),
        password: Some(format!("oauth:mqn0sjxtuojbstj81r37h3dztixk9c")),
        .. Default::default()
    };

    let server = IrcServer::from_config(cfg).unwrap();
    server.identify().unwrap();
    // let server2 = server.clone();

    // thread::spawn(move || {
    //     server2.iter().map(|m| info!("{}", m.unwrap())).count();
    // });
    // loop {
    //     server.send_privmsg("#rocketlobster", "This is a message").unwrap();
    //     thread::sleep(Duration::new(10, 0));
    // }

    for message in server.iter() {
        let message = message.unwrap(); //Just panic
        info!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => if msg.contains("pickles") {
                server.send_privmsg(target, "Hi!").unwrap();
            },
            _ => (),
        }
    }
    
}

fn setup_logging() -> Result<(), fern::InitError> {
    use std::fs;
    match std::fs::metadata("log") {
        Ok(meta) => {
            if !meta.is_dir() {
                panic!("Cannot create file directory \"log\" due to existing file. Please rename or delete the file.");
            }
        },
        Err(e) => {
            fs::DirBuilder::new().recursive(false).create("log").unwrap();
        }
    }

    // NOTE: If file exists, data is APPENDED, not OVERWRITTEN
    let logger_cfg = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, location: &log::LogLocation| {
            format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
        }),
        output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file("log/output.log")],
        level: log::LogLevelFilter::Trace,
    };
    fern::init_global_logger(logger_cfg, log::LogLevelFilter::Trace)
}
