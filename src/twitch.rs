use irc::client;
use irc::client::prelude::*;

pub struct TwitchBot {
    server: client::server::IrcServer
}

impl TwitchBot {
    pub fn new(cfg: client::data::Config) -> TwitchBot {
        let s = IrcServer::from_config(cfg).unwrap();

        TwitchBot {
            server: s
        }
    }

    pub fn run(&self) {
        self.server.identify().unwrap();
        for message in self.server.iter() {
        let message = message.unwrap(); //Just panic
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                // TODO: Take prefix, pull out poster string to use.
                // extract_user() function?
                info!("{}", message);
                if msg.contains("pickles") {
                    self.server.send_privmsg(target, "Hi!").unwrap();
                    info!("{}: Hi!", self.server.config().nickname());
                }
            },
            _ => (),
        }
    }
    }
}