use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    pub(crate) tld: String,
    pub(crate) ip: String,
    pub(crate) name: String,
    pub(crate) secret_key: Option<String>,
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
