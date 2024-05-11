use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user_model::Domain;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Domain>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => "Error loading env variable".to_string(),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Domain> = db.collection("Domain");
        MongoRepo { col }
    }

    pub fn create_domain(&self, new: Domain) -> Result<InsertOneResult, String> {
        let dup = self.get_domain_by_domain(&new.name, &new.domain);

        match dup {
            Err(_) => Err(
                "Domain and name already exist, cannot create duplicate".to_owned(),
            ),
            Ok(_) => {
                let new_doc = Domain {
                    id: None,
                    domain: new.domain,
                    name: new.name,
                    ip: new.ip,
                    secret_key: new.secret_key,
                };
                
                let user = self
                    .col
                    .insert_one(new_doc, None)
                    .expect("Error creating domain");
                Ok(user)
            }
        }
    }


    pub fn get_domain(&self, key: &String) -> Result<Domain, Error> {
        let filter = doc! {"secret_key": key};
        let user_detail = self
            .col
            .find_one(filter, None)
            .expect("Error getting domain's detail");

        let result = user_detail.unwrap();
        
        Ok(result)
    }

    pub fn get_domain_by_domain(&self, name: &String, domain: &String) -> Result<Domain, String> {
        let filter = doc! {"name": name, "domain": domain};
        let user_detail = self
            .col
            .find_one(filter, None)
            .expect("Error getting domain's detail through name and domain");

        let result = user_detail;
        
        match result {
            Some(user) => Ok(user),
            None => Err("Domain not found".to_string()),
        }
    }
    pub fn update_domain(&self, key: &String, new: Domain) -> Result<UpdateResult, Error> {
        let filter = doc! {"secret_key": key};
        let new_doc = doc! {
            "$set":
                {
                    "id": new.id,
                    "domain": new.domain,
                    "name": new.name,
                    "ip": new.ip
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .expect("Error updating domain");
        Ok(updated_doc)
    }

    pub fn delete_domain(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .expect("Error deleting domain");
        Ok(user_detail)
    }

    pub fn get_all_domains(&self) -> Result<Vec<Domain>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .expect("Error getting list of domains");
        let domains = cursors.map(|doc| doc.unwrap()).collect();
        Ok(domains)
    }
}
