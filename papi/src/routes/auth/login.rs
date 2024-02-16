use std::fs::File;
use std::io::Write;
use actix_web::{HttpResponse, post, web};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::{Validate};
use crate::db::Database;
use crate::handshake::{ErrorOrigin, ErrorResponse};
use crate::model::User;
use crate::routes::auth::{authenticate_login, Authenticated};

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/lib/schema/LoginForm.ts")]
#[serde(tag = "login_type")]
pub(crate) enum LoginForm {
  #[serde(rename = "username")]
  WithUsername(LoginFormUsername),
  #[serde(rename = "email")]
  WithEmail(LoginFormEmail)
}

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[ts(export, export_to = "../src/lib/schema/LoginFormUsername.ts")]
struct LoginFormUsername {
  #[validate(length(min = 2, max = 64))]
  username: String,
  #[validate(length(min = 6, max = 64))]
  password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../src/lib/schema/LoginFormEmail.ts")]
struct LoginFormEmail {
  #[validate(email)]
  email: String,
  #[validate(length(min = 6, max = 64))]
  password: String,
}

#[post("/login")]
pub async fn login(post: web::Json<LoginForm>, db: Data<Database>) -> HttpResponse {
  let (user, pass, validated) = match &post.0 {
    LoginForm::WithUsername(u) => (User::get(&db.pool, &u.username).await, u.password.as_str(), u.validate()),
    LoginForm::WithEmail(e) => (User::get_email(&db.pool, &e.email).await, e.password.as_str(), e.validate()),
  };
  
  if let Err(e) = validated {
    return HttpResponse::BadRequest().json(ErrorResponse::validation("Invalid input!", e))
  }

  if user.is_none() { return HttpResponse::BadRequest().json(
    ErrorResponse::public_fatal("User does not exist!", ErrorOrigin::Username))}
  let user = user.unwrap();

  if !user.is_verified() {
    return HttpResponse::Unauthorized().json(ErrorResponse::private_fatal("User is not verified!", ErrorOrigin::User));
  }

  if !user.verify_password(&db.pool, pass).await {
    return HttpResponse::Unauthorized().json(ErrorResponse::public_fatal("Password is incorrect!", ErrorOrigin::Password));
  }

  authenticate_login(&user, &db.pool).await
}