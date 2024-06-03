mod file;
mod structs;

use crate::http::Domain;
use colored::Colorize;
use macros_rs::fmt::{crashln, string};
use mongodb::{options::ClientOptions, Client};
use std::fs::write;
use structs::{Mongo, Server, Settings};

pub use structs::Config;

impl Config {
    pub fn new() -> Self {
        let default_offensive_words = vec!["nigg", "sex", "porn", "igg"];
        let default_tld_list = vec![
            "mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma", "now", "it", "soy", "lol", "uwu", "ohio", "fur",
        ];

        Config {
            config_path: "config.toml".into(),
            server: Server {
                address: "127.0.0.1".into(),
                port: 8080,
                key_db: "storage".into(),
                mongo: Mongo {
                    connection: "".into(),
                    app_name: "DomainApp".into(),
                    db_name: "mydb".into(),
                },
            },
            settings: Settings {
                tld_list: default_tld_list.iter().map(|s| s.to_string()).collect(),
                offensive_words: default_offensive_words.iter().map(|s| s.to_string()).collect(),
            },
        }
    }

    pub fn read(&self) -> Self { file::read(&self.config_path) }
    pub fn get_address(&self) -> String { format!("{}:{}", self.server.address.clone(), self.server.port) }
    pub fn tld_list(&self) -> Vec<&str> { self.settings.tld_list.iter().map(AsRef::as_ref).collect::<Vec<&str>>() }
    pub fn offen_words(&self) -> Vec<&str> { self.settings.offensive_words.iter().map(AsRef::as_ref).collect::<Vec<&str>>() }

    pub fn set_path(&mut self, config_path: &String) -> &mut Self {
        self.config_path = config_path.clone();
        return self;
    }

    pub fn write(&self) -> &Self {
        let contents = match toml::to_string(self) {
            Ok(contents) => contents,
            Err(err) => crashln!("Cannot parse config.\n{}", string!(err).white()),
        };

        if let Err(err) = write(&self.config_path, contents) {
            crashln!("Error writing config to {}.\n{}", self.config_path, string!(err).white())
        }

        log::info!("Created config: {}", &self.config_path,);

        return self;
    }

    pub async fn connect_to_mongo(&self, mongo: &crate::DB) {
        let mut client_options = ClientOptions::parse(&self.server.mongo.connection).await.unwrap_or_default();
        client_options.app_name = Some(self.server.mongo.app_name.clone());

        let client = match Client::with_options(client_options) {
            Ok(client) => client,
            Err(err) => crashln!("Failed to connect to MongoDB.\n{}", string!(err).white()),
        };

        let db = client.database(&self.server.mongo.db_name);
        let collection = db.collection::<Domain>("domains");

        let mut db_lock = mongo.lock().await;
        *db_lock = Some(collection);

        log::info!("MongoDB server connected");
    }
}
