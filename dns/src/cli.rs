use crate::{config::Config, kv, secret, Cli};
use colored::Colorize;
use macros_rs::fmt::{crashln, string};

pub fn list(cli: &Cli) {
    let config = Config::new().set_path(&cli.config).read();

    if let Err(err) = kv::list(&config.server.key_db, false) {
        crashln!("Failed to list: {}", string!(err).white());
    };
}

pub fn create(cli: &Cli, name: &String) {
    let key = secret::generate(60);
    let config = Config::new().set_path(&cli.config).read();

    match kv::set(&config.server.key_db, name, &key) {
        Ok(_) => log::info!("{}\n - name: {}\n - key: {}", "Created key".white(), name.magenta(), key.green()),
        Err(err) => crashln!("Failed to create: {}", string!(err).white()),
    };
}

pub fn remove(cli: &Cli, name: &String) {
    let config = Config::new().set_path(&cli.config).read();

    match kv::remove(&config.server.key_db, name) {
        Ok(_) => log::info!("{} {}", "Deleted key".red(), name.bright_red()),
        Err(err) => crashln!("Failed to delete: {}", string!(err).white()),
    };
}

pub fn info(cli: &Cli, name: &String) {
    let config = Config::new().set_path(&cli.config).read();
    let key = kv::get(&config.server.key_db, name);

    log::info!("{}: {key:?}", name.yellow());
}

pub fn export(cli: &Cli, filename: &String) {
    let config = Config::new().set_path(&cli.config).read();

    match kv::save(&config.server.key_db, filename) {
        Ok(_) => log::info!("Exported keys to {}", filename.green()),
        Err(err) => crashln!("Failed to export: {}", string!(err).white()),
    }
}
