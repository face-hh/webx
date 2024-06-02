use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref LUA_LOGS: Mutex<String> = Mutex::new(String::new());
    pub static ref DNS_SERVER: Mutex<String> = Mutex::new(String::from("api.buss.lol"));
}