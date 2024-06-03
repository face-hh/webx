use crate::TLD;
use axum::{extract::Json, response::IntoResponse};

pub async fn tlds() -> impl IntoResponse {
    // return TLDS as { 1: "bruh", 2: "something" }
    return Json(TLD);
}
