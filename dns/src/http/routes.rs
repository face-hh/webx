use super::models::*;
use crate::{config::Config, kv, secret, DB};
use futures::stream::StreamExt;
use mongodb::{bson::doc, options::FindOptions, Collection};
use regex::Regex;
use std::env;
use std::net::{Ipv4Addr, Ipv6Addr};

use actix_web::{
    web::{self, Data},
    HttpRequest, HttpResponse, Responder,
};

fn validate_ip(domain: &Domain) -> Result<(), HttpResponse> {
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

#[actix_web::get("/")]
pub(crate) async fn index() -> impl Responder {
    HttpResponse::Ok().body(format!(
		  "webxDNS v{}!\n\nThe available endpoints are:\n\n - [GET] /domains\n - [GET] /domain/{{name}}/{{tld}}\n - [POST] /domain\n - [PUT] /domain/{{key}}\n - [DELETE] /domain/{{key}}\n - [GET] /tlds\n\nRatelimits are as follows: 3 requests per 5 minutes on `[POST] /domain`.\n\nCode link: https://github.com/face-hh/webx/tree/master/dns",env!("CARGO_PKG_VERSION")),
	 )
}

pub(crate) async fn create_logic(domain: Domain, config: &Config, collection: &Collection<Domain>) -> Result<Domain, HttpResponse> {
    validate_ip(&domain)?;

    if !config.tld_list().contains(&domain.tld.as_str()) || !domain.name.chars().all(|c| c.is_alphabetic() || c == '-') || domain.name.len() > 24 {
        return Err(HttpResponse::BadRequest().json(Error {
            msg: "Failed to create domain",
            error: "Invalid name, non-existent TLD, or name too long (24 chars).".into(),
        }));
    }

    if config.offen_words().iter().any(|word| domain.name.contains(word)) {
        return Err(HttpResponse::BadRequest().json(Error {
            msg: "Failed to create domain",
            error: "The given domain name is offensive.".into(),
        }));
    }

    let existing_domain = collection
        .find_one(doc! { "name": &domain.name, "tld": &domain.tld }, None)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    if existing_domain.is_some() {
        return Err(HttpResponse::Conflict().finish());
    }

    collection.insert_one(&domain, None).await.map_err(|_| HttpResponse::Conflict().finish())?;

    Ok(domain)
}

pub(crate) async fn create_domain(domain: web::Json<Domain>, config: Data<Config>) -> impl Responder {
    let secret_key = secret::generate(31);
    let mut domain = domain.into_inner();
    domain.secret_key = Some(secret_key);

    let collection = DB.lock().await;
    let collection = match collection.as_ref() {
        Some(res) => res,
        None => {
            return HttpResponse::InternalServerError().json(Error {
                msg: "Failed to fetch collection",
                error: "Unknown Error".into(),
            })
        }
    };

    match create_logic(domain, &config, collection).await {
        Ok(domain) => HttpResponse::Ok().json(domain),
        Err(error) => error,
    }
}

#[actix_web::post("/registry/domain")]
pub(crate) async fn elevated_domain(domain: web::Json<Domain>, config: Data<Config>, req: HttpRequest) -> impl Responder {
    match super::get_token(&req) {
        Ok((name, key)) => match kv::get(&config.server.key_db, &name.to_string()) {
            Ok(value) => macros_rs::exp::then!(
                value != key,
                return HttpResponse::Unauthorized().json(Error {
                    msg: "Invalid authorization header",
                    error: "Token is invalid".into(),
                })
            ),
            Err(err) => {
                return HttpResponse::InternalServerError().json(Error {
                    msg: "Failed to fetch authorization header",
                    error: err.to_string(),
                })
            }
        },
        Err(err) => {
            return HttpResponse::Unauthorized().json(Error {
                msg: "Authorization failed",
                error: err.to_string(),
            })
        }
    };

    let secret_key = secret::generate(31);
    let mut domain = domain.into_inner();
    domain.secret_key = Some(secret_key);

    let collection = DB.lock().await;
    let collection = match collection.as_ref() {
        Some(res) => res,
        None => {
            return HttpResponse::InternalServerError().json(Error {
                msg: "Failed to fetch collection",
                error: "Unknown Error".into(),
            })
        }
    };

    match create_logic(domain, &config, collection).await {
        Ok(domain) => HttpResponse::Ok().json(domain),
        Err(error) => error,
    }
}

#[actix_web::get("/domain/{name}/{tld}")]
pub(crate) async fn get_domain(path: web::Path<(String, String)>) -> impl Responder {
    let collection = DB.lock().await;
    let collection = match collection.as_ref() {
        Some(res) => res,
        None => {
            return HttpResponse::InternalServerError().json(Error {
                msg: "Failed to fetch collection",
                error: "Unknown Error".into(),
            })
        }
    };

    let (name, tld) = path.into_inner();
    let filter = doc! { "name": name, "tld": tld };

    match collection.find_one(filter, None).await {
        Ok(Some(domain)) => HttpResponse::Ok().json(ResponseDomain {
            tld: domain.tld,
            name: domain.name,
            ip: domain.ip,
        }),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::put("/domain/{key}")]
pub(crate) async fn update_domain(path: web::Path<String>, domain_update: web::Json<UpdateDomain>) -> impl Responder {
    let collection = DB.lock().await;
    let collection = match collection.as_ref() {
        Some(res) => res,
        None => {
            return HttpResponse::InternalServerError().json(Error {
                msg: "Failed to fetch collection",
                error: "Unknown Error".into(),
            })
        }
    };

    let key = path.into_inner();
    let filter = doc! { "secret_key": key };
    let update = doc! { "$set": { "ip": &domain_update.ip } };

    match collection.update_one(filter, update, None).await {
        Ok(result) => {
            if result.matched_count == 1 {
                HttpResponse::Ok().json(domain_update.into_inner())
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::delete("/domain/{key}")]
pub(crate) async fn delete_domain(path: web::Path<String>) -> impl Responder {
    let collection = DB.lock().await;
    let collection = match collection.as_ref() {
        Some(res) => res,
        None => {
            return HttpResponse::InternalServerError().json(Error {
                msg: "Failed to fetch collection",
                error: "Unknown Error".into(),
            })
        }
    };

    let key = path.into_inner();
    let filter = doc! { "secret_key": key };

    match collection.delete_one(filter, None).await {
        Ok(result) => {
            if result.deleted_count == 1 {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::get("/domains")]
pub(crate) async fn get_domains(query: web::Query<PaginationParams>) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let limit = query.page_size.unwrap_or(15);

    if page == 0 || limit == 0 || limit == 100 {
        return HttpResponse::BadRequest().json(Error {
            msg: "page_size must be greater than 0 and less than 100",
            error: "Invalid pagination parameters".into(),
        });
    }

    let collection = DB.lock().await;

    let collection = match collection.as_ref() {
        Some(res) => res,
        None => {
            return HttpResponse::InternalServerError().json(Error {
                msg: "Failed to fetch collection",
                error: "Unknown Error".into(),
            })
        }
    };

    let skip = (page - 1) * limit;
    let find_options = FindOptions::builder().skip(Some(skip as u64)).limit(Some(limit as i64)).build();

    let cursor = match collection.find(None, find_options).await {
        Ok(res) => res,
        Err(err) => {
            return HttpResponse::InternalServerError().json(Error {
                msg: "Failed to fetch cursor",
                error: err.to_string(),
            })
        }
    };

    let domains: Vec<ResponseDomain> = cursor
        .filter_map(|result| async {
            match result {
                Ok(domain) => Some(ResponseDomain {
                    tld: domain.tld,
                    name: domain.name,
                    ip: domain.ip,
                }),
                Err(_) => None,
            }
        })
        .collect()
        .await;

    HttpResponse::Ok().json(PaginationResponse { domains, page, limit })
}

#[actix_web::get("/tlds")]
pub(crate) async fn get_tlds(config: Data<Config>) -> impl Responder { HttpResponse::Ok().json(&*config.tld_list()) }
