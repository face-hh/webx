use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(skip)]
    pub config_path: String,
    pub(crate) server: Server,
    pub(crate) settings: Settings,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub(crate) address: String,
    pub(crate) port: u64,
    pub(crate) mongo: Mongo,
    pub(crate) key_db: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mongo {
    pub(crate) connection: String,
    pub(crate) app_name: String,
    pub(crate) db_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub(crate) tld_list: Vec<String>,
    pub(crate) offensive_words: Vec<String>,
}
