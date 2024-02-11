pub mod permission;
pub mod user;

use actix_web::{get, web};
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
  pub message: String,
}

#[get("/")]
async fn index() -> web::Json<MessageResponse> {
  web::Json(MessageResponse {
    message: "Welcome, This is the Joco Svelte Template!".to_string()
  })
}