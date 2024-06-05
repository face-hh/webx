use super::helpers::deserialize_lowercase;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    pub(crate) ip: String,
    pub(crate) secret_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lowercase")]
    pub(crate) tld: String,
    #[serde(deserialize_with = "deserialize_lowercase")]
    pub(crate) name: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct ResponseDomain {
    pub(crate) tld: String,
    pub(crate) ip: String,
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UpdateDomain {
    pub(crate) ip: String,
}

#[derive(Serialize)]
pub(crate) struct Error {
    pub(crate) msg: &'static str,
    pub(crate) error: String,
}

#[derive(Serialize)]
pub(crate) struct Ratelimit {
    pub(crate) msg: String,
    pub(crate) error: &'static str,
    pub(crate) after: u64,
}

#[derive(Deserialize)]
pub(crate) struct PaginationParams {
    #[serde(alias = "p", alias = "doc")]
    pub(crate) page: Option<u32>,
    #[serde(alias = "s", alias = "size", alias = "l", alias = "limit")]
    pub(crate) page_size: Option<u32>,
}

#[derive(Serialize)]
pub(crate) struct PaginationResponse {
    pub(crate) domains: Vec<ResponseDomain>,
    pub(crate) page: u32,
    pub(crate) limit: u32,
}

#[derive(Deserialize)]
pub(crate) struct DomainQuery {
    pub(crate) name: String,
    pub(crate) tld: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct DomainList {
    pub(crate) domain: String,
    pub(crate) taken: bool,
}
