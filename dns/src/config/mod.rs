mod file;
mod structs;

use colored::Colorize;
use macros_rs::fmt::{crashln, string};
use std::fs::write;
use structs::{Mongo, Server, Settings};

pub use structs::Config;

pub fn read() -> Config {
    let config_path = format!("config.toml");

    let default_offensive_words = vec!["nigg", "sex", "porn"];
    let default_tld_list = vec![
        "mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma", "now", "it", "soy", "lol", "uwu", "ohio", "fur",
    ];

    if !file::Exists::check(&config_path).file() {
        let config = Config {
            server: Server {
                address: string!("127.0.0.1"),
                port: 8080,
                mongo: Mongo {
                    connection: string!(""),
                    app_name: string!("DomainApp"),
                    db_name: string!("mydb"),
                },
            },
            settings: Settings {
                tld_list: default_tld_list.iter().map(|s| s.to_string()).collect(),
                offensive_words: default_offensive_words.iter().map(|s| s.to_string()).collect(),
            },
        };

        let contents = match toml::to_string(&config) {
            Ok(contents) => contents,
            Err(err) => crashln!("Cannot parse config.\n{}", string!(err).white()),
        };

        if let Err(err) = write(&config_path, contents) {
            crashln!("Error writing config.\n{}", string!(err).white())
        }
    }

    file::read(config_path)
}

impl Config {
    pub fn get_address(&self) -> String { format!("{}:{}", self.server.address.clone(), self.server.port) }
    pub fn tld_list(&self) -> Vec<&str> { self.settings.tld_list.iter().map(AsRef::as_ref).collect::<Vec<&str>>() }
    pub fn offen_words(&self) -> Vec<&str> { self.settings.offensive_words.iter().map(AsRef::as_ref).collect::<Vec<&str>>() }
}
