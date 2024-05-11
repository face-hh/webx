use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Domain {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub domain: String,
    pub ip: String,
    pub name: String,
    pub secret_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainInput {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub domain: String,
    pub ip: String,
    pub name: String,
}