use colored::Colorize;
use macros_rs::fmt::{crashln, string};
use std::{fs, path::Path};

pub fn read<T: serde::de::DeserializeOwned>(path: String) -> T {
    let contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) => crashln!("Cannot find config.\n{}", string!(err).white()),
    };

    match toml::from_str(&contents).map_err(|err| string!(err)) {
        Ok(parsed) => parsed,
        Err(err) => crashln!("Cannot parse config.\n{}", err.white()),
    }
}

pub struct Exists<'p> {
    path: &'p str,
}

impl<'p> Exists<'p> {
    pub fn check(path: &'p str) -> Self { Self { path } }
    pub fn file(&self) -> bool { Path::new(self.path).exists() }
}
