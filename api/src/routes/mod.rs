
pub mod users;
pub mod auth;

use actix_web::{get, HttpResponse};
use api_lib::handshake::{OkResponse, OkResponseKind};
use api_proc::endpoint;

#[endpoint("", "get", (), (), ())]
#[get("/")]
async fn index() -> HttpResponse {
  OkResponse::new_send("Welcome, This is the Joco Svelte Template!", OkResponseKind::<()>::Simple)
}
