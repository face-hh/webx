use std::sync::Mutex;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LUA_LOGS: Mutex<String> = Mutex::new(String::new());
}

lazy_static! {
    pub static ref URI_PARAMETERS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}