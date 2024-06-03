use super::{models::*, DB};
use crate::{config::Config, secret};
use futures::stream::StreamExt;
use mongodb::bson::doc;

use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};

#[actix_web::get("/")]
pub(crate) async fn index() -> impl Responder {
    HttpResponse::Ok().body(
		  "Hello, world! The available endpoints are:\nGET /domains,\nGET /domain/{name}/{tld},\nPOST /domain,\nPUT /domain/{key},\nDELETE /domain/{key},\nGET /tlds.\nRatelimits are as follows: 10 requests per 60s.\n",
	 )
}

pub(crate) async fn create_domain(domain: web::Json<Domain>, config: Data<Config>) -> impl Responder {
    let secret_key = secret::generate(31);
    let mut domain = domain.into_inner();
    domain.secret_key = Some(secret_key.clone());

    if !config.tld_list().contains(&domain.tld.as_str()) || !domain.name.chars().all(|c| c.is_alphabetic() || c == '-') || domain.name.len() > 24 {
        return HttpResponse::BadRequest().body("Invalid name, non-existent TLD, or name too long (24 chars).");
    }

    if config.offen_words().iter().any(|word| domain.name.contains(word)) {
        return HttpResponse::BadRequest().body("The given domain is offensive.");
    }

    let collection = DB.lock().await;
    let collection = collection.as_ref().unwrap();
    let existing_domain = collection.find_one(doc! { "name": &domain.name, "tld": &domain.tld }, None).await.unwrap();

    if existing_domain.is_some() {
        return HttpResponse::Conflict().finish();
    }

    let insert_result = collection.insert_one(domain.clone(), None).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json(domain),
        Err(_) => HttpResponse::Conflict().finish(),
    }
}

#[actix_web::get("/domain/{name}/{tld}")]
pub(crate) async fn get_domain(path: web::Path<(String, String)>) -> impl Responder {
    let collection = DB.lock().await;
    let collection = collection.as_ref().unwrap();

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
    let collection = collection.as_ref().unwrap();

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
    let collection = collection.as_ref().unwrap();

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
pub(crate) async fn get_domains() -> impl Responder {
    let collection = DB.lock().await;
    let collection = collection.as_ref().unwrap();
    let cursor = collection.find(None, None).await.unwrap();

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

    HttpResponse::Ok().json(domains)
}

#[actix_web::get("/tlds")]
pub(crate) async fn get_tlds(config: Data<Config>) -> impl Responder { HttpResponse::Ok().json(&*config.tld_list()) }
