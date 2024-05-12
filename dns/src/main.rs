mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use rocket_governor::rocket_governor_catcher;

use api::domain_api::{create_domain, delete_domain, get_all_domains, get_domain, update_domain};

use rocket_governor::{Method, Quota, RocketGovernable};

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: Method, route_name: &str) -> Quota {
        match route_name {
            "get_domain" => Quota::per_second(Self::nonzero(1u32)),
            "delete_domain" => Quota::per_second(Self::nonzero(1u32)),
            "update_domain" => Quota::per_minute(Self::nonzero(30u32)),
            "create_domain" => Quota::per_hour(Self::nonzero(1u32)),
            "get_all_domains" => Quota::per_hour(Self::nonzero(1u32)),
            _ => Quota::per_second(Self::nonzero(1u32)),
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(repository::mongodb_repo::MongoRepo::init())
        .mount("/", routes![create_domain])
        .mount("/", routes![get_domain])
        .mount("/", routes![update_domain])
        .mount("/", routes![delete_domain])
        .mount("/", routes![get_all_domains])
        .register("/", catchers![rocket_governor_catcher])
}
