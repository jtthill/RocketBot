use std::fs;
use irc;
use std::default::Default;

pub fn init_configs() -> irc::client::data::Config {
    // cfg directory checks
    match fs::metadata("cfg") {
        Ok(meta) => {
            if !meta.is_dir() {
                panic!("Cannot create file directory \"cfg\" due to existing file. Please rename or delete the file.");
            }
        },
        Err(_) => {
            debug!("Creating cfg directory.");
            fs::DirBuilder::new().recursive(false).create("cfg").unwrap();
        }
    }

    match fs::metadata("cfg/server.json") {
        Ok(_) => {
            // File exists, load config from that
            debug!("Loading irc config from JSON file.");
            irc::client::data::Config::load("cfg/server.json").unwrap()
        },
        Err(_) => {
            // File doesn't exist, create new config to save to.
            create_irc_config()
        }
    }
}

fn create_irc_config() -> irc::client::data::Config {
    println!("Creating new configuration for RocketBot.");
    // TODO: Add command line config building from user if file 
    // doesn't exist
    let cfg = irc::client::data::Config {
        nickname: Some(format!("rocketlobot")),
        server: Some(format!("irc.chat.twitch.tv")),
        port: Some(6667),
        channels: Some(vec![format!("#rocketlobster")]),
        password: Some(format!("oauth:mqn0sjxtuojbstj81r37h3dztixk9c")),
        .. Default::default()
    };
    cfg.save("cfg/server.json").unwrap();
    cfg
}