use crate::{CLIENT, TLD};
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use mongodb::bson::doc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    pub tld: String,
    pub name: String,
    pub ip: String,
    pub secret_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct DomainIncoming {
    name: String,
    tld: String,
    ip: String,
}

#[derive(Serialize, Deserialize)]
pub struct Update {
    ip: String,
    key: String,
}

#[derive(Serialize, Deserialize)]
struct SecretKey {
    secret_key: String,
}

pub async fn domain_register(Json(payload): Json<DomainIncoming>) -> impl IntoResponse {
    let secret_key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();

    // regex: /^[a-zA-Z\-]+$/
    if !(payload.name.chars().all(|c| c.is_alphabetic() || c == '-'))
        || !TLD.contains(&payload.tld.as_str())
        || payload.name.len() > 24
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!( {
                "error" : "Error, your name/tld doesnt fit the criteria.".to_string(),
            })),
        )
            .into_response();
    }

    // check for common swears and slurs
    if payload.name.to_lowercase().contains("sex")
        || payload.name.to_lowercase().contains("porn")
        || payload.name.to_lowercase().contains("nigg")
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error" : "Error, your name/tld doesnt fit the criteria.".to_string()
            })),
        )
            .into_response();
    }

    let db = CLIENT.get().await.database("mydb");
    let collection: mongodb::Collection<Domain> = db.collection("domains");
    let filter = doc! { "name": &payload.name, "tld": &payload.tld };
    let result = collection.find_one(filter, None).await;

    if result.unwrap().is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error" : "Error, domain already in use.".to_string(),
            })),
        )
            .into_response();
    }

    let domain = Domain {
        tld: payload.tld,
        name: payload.name,
        ip: payload.ip,
        secret_key: secret_key.clone(),
    };

    collection
        .insert_one(domain, None)
        .await
        .expect("Error inserting domain.");

    return (StatusCode::OK, Json(SecretKey { secret_key })).into_response();
}

// app.get('/domain/:name/:tld', async (req, res)
pub async fn get_domain(Path((name, tld)): Path<(String, String)>) -> impl IntoResponse {
    let db = CLIENT.get().await.database("mydb");
    let collection: mongodb::Collection<Domain> = db.collection("domains");
    let filter = doc! { "name": name, "tld": tld };
    let result = collection.find_one(filter, None).await;

    if result.is_err() {
        println!("Error: {:?}", result);
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error" : "Unknown db error!.".to_string(),
            })),
        )
            .into_response();
    }

    let result = result.unwrap();
    if result.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error" : "Error, domain not found.".to_string(),
            })),
        )
            .into_response();
    }

    let domain = result.unwrap();
    return (
        StatusCode::OK,
        Json(json!(
            {
                "name": domain.name,
                "tld": domain.tld,
                "ip": domain.ip,
        })),
    )
        .into_response();
}

//app.put('/domain/:key', async (req, res)

pub async fn update_domain(Json(payload): Json<Update>) -> impl IntoResponse {
    // update the domain with the new ip
    let db = CLIENT.get().await.database("mydb");
    let collection: mongodb::Collection<Domain> = db.collection("domains");
    let filter = doc! { "secret_key": payload.key };

    let result = collection.find_one(filter.clone(), None).await;
    if result.unwrap().is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error" : "Error, domain not found.".to_string(),
            })),
        )
            .into_response();
    }

    let update = doc! { "$set": { "ip": payload.ip } };
    collection
        .update_one(filter, update, None)
        .await
        .expect("An error occured while updating the domain.");

    return (
        StatusCode::OK,
        Json(json!({ "message": "Domain updated." })),
    )
        .into_response();
}

// app.delete('/domain/:id', async (req, res)
pub async fn delete_domain(Path(key): Path<String>) -> impl IntoResponse {
    let db = CLIENT.get().await.database("mydb");
    let collection: mongodb::Collection<Domain> = db.collection("domains");
    let filter = doc! { "secret_key": key };

    let result = collection.find_one(filter.clone(), None).await;
    if result.unwrap().is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error" : "Error, domain not found.".to_string(),
            })),
        )
            .into_response();
    }

    collection
        .delete_one(filter, None)
        .await
        .expect("An error occured while deleting the domain.");

    return (
        StatusCode::OK,
        Json(json!({ "message": "Domain deleted." })),
    )
        .into_response();
}

pub async fn domain_list() -> impl IntoResponse {
    let db = CLIENT.get().await.database("mydb");
    let collection: mongodb::Collection<Domain> = db.collection("domains");
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut domains: Vec<DomainIncoming> = Vec::new();
    while cursor.advance().await.unwrap() {
        let domain = cursor.deserialize_current().unwrap();
        domains.push(DomainIncoming {
            name: domain.name,
            tld: domain.tld,
            ip: domain.ip,
        });
    }

    return (StatusCode::OK, Json(domains)).into_response();
}
