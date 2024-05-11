use crate::{models::user_model::{Domain, DomainInput}, repository::mongodb_repo::MongoRepo};
use rand::Rng;
use rocket::{http::Status, serde::json::Json, State};

#[post("/domain", data = "<new>")]
pub fn create_domain(
    db: &State<MongoRepo>,
    new: Json<DomainInput>,
) -> Result<Json<Domain>, Status> {
    let secret_key = generate_api_key(24);
    println!("{secret_key}");
    let data = Domain {
        id: None,
        domain: new.domain.to_owned(),
        ip: new.ip.to_owned(),
        name: new.name.to_owned(),
        secret_key,
    };

    let res = db.create_domain(data.clone());
    match res {
        Ok(_) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/domain/<name>/<domain>")]
pub fn get_domain(db: &State<MongoRepo>, name: String, domain: String) -> Result<Json<DomainInput>, Status> {
    if name.is_empty() || domain.is_empty() {
        return Err(Status::BadRequest);
    };

    let res = db.get_domain_by_domain(&name, &domain);

    match res {
        Ok(domain) => {
            Ok(Json(DomainInput {
                id: None,
                domain: domain.domain.to_owned(),
                name: domain.name.to_owned(),
                ip: domain.ip.to_owned(),
            }))
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[put("/domain/<key>", data = "<new>")]
pub fn update_domain(
    db: &State<MongoRepo>,
    key: String,
    new: Json<DomainInput>,
) -> Result<Json<DomainInput>, Status> {
    if key.is_empty() {
        return Err(Status::BadRequest);
    };

    let data = Domain {
        id: None,
        domain: new.domain.to_owned(),
        name: new.name.to_owned(),
        ip: new.ip.to_owned(),
        secret_key: key.to_owned(),
    };

    if let Ok(domain_info) = db.get_domain(&key) {
        if domain_info.secret_key != key {
            return Err(Status::Forbidden);
        }
    } else {
        return Err(Status::NotFound);
    }

    let update_result = db.update_domain(&key, data);
    match update_result {
        Ok(_) => {
            let domain = db.get_domain(&key).unwrap();

            Ok(Json(DomainInput {
                id: None,
                domain: domain.domain.to_owned(),
                name: domain.name.to_owned(),
                ip: domain.ip.to_owned(),
            }))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/domain/<path>")]
pub fn delete_domain(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_domain(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Domain successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/domains")]
pub fn get_all_domains(db: &State<MongoRepo>) -> Result<Json<Vec<DomainInput>>, Status> {
    let domains = db.get_all_domains();
    match domains {
        Ok(mut domains) => {
            let converted_domains: Vec<DomainInput> = domains
                .iter_mut()
                .map(|domain| DomainInput {
                    id: None,
                    domain: domain.domain.to_owned(),
                    name: domain.name.to_owned(),
                    ip: domain.ip.to_owned(),
                })
                .collect();
            Ok(Json(converted_domains))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

// MISC

fn generate_api_key(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    let key: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    key
}
