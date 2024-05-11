mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::domain_api::{create_domain, get_domain, update_domain, delete_domain, get_all_domains};

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(repository::mongodb_repo::MongoRepo::init())
        .mount("/", routes![create_domain])
        .mount("/", routes![get_domain])
        .mount("/", routes![update_domain])
        .mount("/", routes![delete_domain])
        .mount("/", routes![get_all_domains])
}
