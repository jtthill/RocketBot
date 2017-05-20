use irc::client;
use irc::client::prelude::*;
use user::User;

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
        // Request tags be sent to identify users
        self.server.send("CAP REQ :twitch.tv/tags\r\n").unwrap();
        for message in self.server.iter() {
            let message = message.unwrap(); //Just panic
            match message.command {
                Command::PRIVMSG(ref target, ref msg) => {
                    // User struct contains all relevant info about sender
                    let user = User::new(&message);
                    info!("{}: {}", user.display_name, msg);
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
