use super::{models::*, AppState};
use actix_web::{web::Data, HttpResponse};
use mongodb::bson::doc;
use regex::Regex;
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

pub fn validate_ip(domain: &Domain) -> Result<(), HttpResponse> {
    let http_regex = Regex::new(r"^https?://[a-zA-Z0-9.-]+$").unwrap();

    let is_valid_ip = domain.ip.parse::<Ipv4Addr>().is_ok() || domain.ip.parse::<Ipv6Addr>().is_ok();
    let is_valid_http = http_regex.is_match(&domain.ip);

    if is_valid_ip || is_valid_http {
        if domain.name.len() <= 100 {
            Ok(())
        } else {
            Err(HttpResponse::BadRequest().json(Error {
                msg: "Failed to create domain",
                error: "Invalid name, non-existent TLD, or name too long (100 chars).".into(),
            }))
        }
    } else {
        Err(HttpResponse::BadRequest().json(Error {
            msg: "Failed to create domain",
            error: "Invalid name, non-existent TLD, or name too long (100 chars).".into(),
        }))
    }
}

pub fn deserialize_lowercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.to_lowercase())
}

pub async fn is_domain_taken(name: &str, tld: Option<&str>, app: Data<AppState>) -> Vec<DomainList> {
    if let Some(tld) = tld {
        let filter = doc! { "name": name, "tld": tld };
        let taken = app.db.find_one(filter, None).await.unwrap().is_some();

        vec![DomainList {
            taken,
            domain: format!("{}.{}", name, tld),
        }]
    } else {
        let mut result = Vec::new();
        for tld in &*app.config.tld_list() {
            let filter = doc! { "name": name, "tld": tld };
            let taken = app.db.find_one(filter, None).await.unwrap().is_some();

            result.push(DomainList {
                taken,
                domain: format!("{}.{}", name, tld),
            });
        }
        result
    }
}
