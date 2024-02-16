
use actix_web::{HttpResponse, post, web};
use actix_web::web::{Data};
use serde::{Deserialize};
use validator::Validate;
use crate::db::Database;
use crate::env::ApiEnv;
use crate::handshake::{ErrorOrigin, ErrorResponse, OkResponse, OkResponseKind};
use crate::model::User;
use crate::routes::auth::authenticate_login;

#[derive(Deserialize, Validate)]
struct RegisterForm {
  #[validate(length(min = 2, max = 64))]
  username: String,
  #[validate(email)]
  email: String,
  #[validate(length(min = 6, max = 64))]
  password: String
}

#[post("/register")]
pub async fn register(post: web::Json<RegisterForm>, db: Data<Database>) -> HttpResponse {
  if let Some(_) = User::get(&db.pool, &post.username).await {
    return HttpResponse::Conflict().json(ErrorResponse::public_fatal("User already exists!", ErrorOrigin::User));
  }

  let registered = User::register(&db.pool, &*post.username, &*post.email, &*post.password).await;
  if let Err(e) = registered {
    return HttpResponse::InternalServerError().json(ErrorResponse::private_fatal(format!("Could not register!: {e}").as_str(), ErrorOrigin::Db));
  }
  let mut registered = registered.unwrap();

  if ApiEnv::email_auth_enabled() {
    //todo mail SMTP
    OkResponse::new_send("Redirecting to mailer", OkResponseKind::<()>::Redirected {
      to: "todo".to_string()
    })
  } else {
    if let Err(_) = registered.set_verified(&db.pool, true).await {
      return HttpResponse::InternalServerError().json(ErrorResponse::private_fatal("Could not set user to verified!", ErrorOrigin::Db));
    }

    authenticate_login(&registered, &db.pool).await
  }
}

