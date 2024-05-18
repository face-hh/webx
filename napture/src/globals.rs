use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref LUA_LOGS: Mutex<String> = Mutex::new(String::new());
}