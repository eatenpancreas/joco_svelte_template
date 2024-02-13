
pub mod users;
pub mod auth;

use actix_web::{get, HttpResponse};
use crate::handshake::{OkKind, OkResponse};

#[get("/")]
async fn index() -> HttpResponse {
  OkResponse::new_send("Welcome, This is the Joco Svelte Template!", OkKind::<()>::Simple)
}