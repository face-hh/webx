use super::{models::*, AppState};
use crate::{http::helpers, kv, secret};
use futures::stream::StreamExt;
use mongodb::{bson::doc, options::FindOptions};
use std::env;

use actix_web::{
    web::{self, Data},
    HttpRequest, HttpResponse, Responder,
};

#[actix_web::get("/")]
pub(crate) async fn index() -> impl Responder {
    HttpResponse::Ok().body(format!(
		  "webxDNS v{}!\n\nThe available endpoints are:\n\n - [GET] /domains\n - [GET] /domain/{{name}}/{{tld}}\n - [POST] /domain\n - [PUT] /domain/{{key}}\n - [DELETE] /domain/{{key}}\n - [GET] /tlds\n\nRatelimits are as follows: 5 requests per 10 minutes on `[POST] /domain`.\n\nCode link: https://github.com/face-hh/webx/tree/master/dns",env!("CARGO_PKG_VERSION")),
	 )
}

pub(crate) async fn create_logic(domain: Domain, app: &AppState) -> Result<Domain, HttpResponse> {
    helpers::validate_ip(&domain)?;

    if !app.config.tld_list().contains(&domain.tld.as_str()) || !domain.name.chars().all(|c| c.is_alphabetic() || c == '-') || domain.name.len() > 24 {
        return Err(HttpResponse::BadRequest().json(Error {
            msg: "Failed to create domain",
            error: "Invalid name, non-existent TLD, or name too long (24 chars).".into(),
        }));
    }

    if app.config.offen_words().iter().any(|word| domain.name.contains(word)) {
        return Err(HttpResponse::BadRequest().json(Error {
            msg: "Failed to create domain",
            error: "The given domain name is offensive.".into(),
        }));
    }

    let existing_domain = app
        .db
        .find_one(doc! { "name": &domain.name, "tld": &domain.tld }, None)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    if existing_domain.is_some() {
        return Err(HttpResponse::Conflict().finish());
    }

    app.db.insert_one(&domain, None).await.map_err(|_| HttpResponse::Conflict().finish())?;

    Ok(domain)
}

pub(crate) async fn create_domain(domain: web::Json<Domain>, app: Data<AppState>) -> impl Responder {
    let secret_key = secret::generate(31);
    let mut domain = domain.into_inner();
    domain.secret_key = Some(secret_key);

    match create_logic(domain, app.as_ref()).await {
        Ok(domain) => HttpResponse::Ok().json(domain),
        Err(error) => error,
    }
}

#[actix_web::post("/registry/domain")]
pub(crate) async fn elevated_domain(domain: web::Json<Domain>, app: Data<AppState>, req: HttpRequest) -> impl Responder {
    match super::get_token(&req) {
        Ok((name, key)) => match kv::get(&app.config.server.key_db, &name.to_string()) {
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

    match create_logic(domain, app.as_ref()).await {
        Ok(domain) => HttpResponse::Ok().json(domain),
        Err(error) => error,
    }
}

#[actix_web::get("/domain/{name}/{tld}")]
pub(crate) async fn get_domain(path: web::Path<(String, String)>, app: Data<AppState>) -> impl Responder {
    let (name, tld) = path.into_inner();
    let filter = doc! { "name": name, "tld": tld };

    match app.db.find_one(filter, None).await {
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
pub(crate) async fn update_domain(path: web::Path<String>, domain_update: web::Json<UpdateDomain>, app: Data<AppState>) -> impl Responder {
    let key = path.into_inner();
    let filter = doc! { "secret_key": key };
    let update = doc! { "$set": { "ip": &domain_update.ip } };

    match app.db.update_one(filter, update, None).await {
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
pub(crate) async fn delete_domain(path: web::Path<String>, app: Data<AppState>) -> impl Responder {
    let key = path.into_inner();
    let filter = doc! { "secret_key": key };

    match app.db.delete_one(filter, None).await {
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

#[actix_web::post("/domain/check")]
pub(crate) async fn check_domain(query: web::Json<DomainQuery>, app: Data<AppState>) -> impl Responder {
    let DomainQuery { name, tld } = query.into_inner();

    let result = helpers::is_domain_taken(&name, tld.as_deref(), app).await;
    HttpResponse::Ok().json(result)
}

#[actix_web::get("/domains")]
pub(crate) async fn get_domains(query: web::Query<PaginationParams>, app: Data<AppState>) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let limit = query.page_size.unwrap_or(15);

    if page == 0 || limit == 0 {
        return HttpResponse::BadRequest().json(Error {
            msg: "page_size or page must be greater than 0",
            error: "Invalid pagination parameters".into(),
        });
    }

    if limit > 100 {
        return HttpResponse::BadRequest().json(Error {
            msg: "page_size must be greater than 0 and less than or equal to 100",
            error: "Invalid pagination parameters".into(),
        });
    }

    let skip = (page - 1) * limit;
    let find_options = FindOptions::builder().skip(Some(skip as u64)).limit(Some(limit as i64)).build();

    let cursor = match app.db.find(None, find_options).await {
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
pub(crate) async fn get_tlds(app: Data<AppState>) -> impl Responder { HttpResponse::Ok().json(&*app.config.tld_list()) }
