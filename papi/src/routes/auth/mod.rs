pub(crate) mod login;
mod register;

use actix_web::{HttpResponse, web};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use ts_rs::TS;
use crate::env::ApiEnv;
use crate::handshake::{ErrorOrigin, ErrorResponse, OkResponse, OkResponseKind};
use crate::model::User;
use crate::routes::auth::login::login as login_route;
use crate::routes::auth::register::register as register_route;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/auth")
      .service(login_route)
      .service(register_route)
  );
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/lib/schema/Authenticated.ts")]
pub struct Authenticated {
  token: String,
  username: String,
}

pub async fn authenticate_login(user: &User, pool: &PgPool) -> HttpResponse {
  match user.login(pool).await {
    Err(e) => HttpResponse::InternalServerError().json(
      ErrorResponse::private_fatal(format!("User not logged in: {e}").as_str(), ErrorOrigin::Db)),
    Ok(auth) => {
      let secret = ApiEnv::jwt_secret();
      let jwt = jsonwebtoken::encode(&Header::default(), &auth, &EncodingKey::from_secret(secret.as_bytes()));

      if jwt.is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponse::private_fatal("Could not encode authorization", ErrorOrigin::Auth));
      }
      let jwt = jwt.unwrap();

      OkResponse::new_send("Logged in!", OkResponseKind::Data(Authenticated {
        token: jwt,
        username: user.username().clone()
      }))
    }
  }
}

// #[post("/confirm")]
// fn confirm(post: web::Json<NewPost>, db: Data<Database>) -> HttpResponse {
// }