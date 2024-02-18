
pub mod users;
pub mod auth;

use actix_web::{get, HttpResponse};
use api_proc::endpoint;
use crate::handshake::{OkResponseKind, OkResponse};

#[endpoint("", "get", (), ())]
#[get("/")]
async fn index() -> HttpResponse {
  OkResponse::new_send("Welcome, This is the Joco Svelte Template!", OkResponseKind::<()>::Simple)
}