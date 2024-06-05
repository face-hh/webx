use std::sync::{Mutex, Arc};
use glib::Source;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref LUA_LOGS: Mutex<String> = Mutex::new(String::new());
    pub static ref DNS_SERVER: Mutex<String> = Mutex::new(String::from("https://api.buss.lol"));
    pub static ref LUA_TIMEOUTS: Arc<Mutex<Vec<Source>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref APPDATA_PATH: Mutex<String> = Mutex::new(String::from("scoopity poop"));
}
