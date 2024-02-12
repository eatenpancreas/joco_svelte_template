use actix_web::{HttpResponse, post, web};
use actix_web::web::{Data};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::db::Database;
use crate::env::ApiEnv;
use crate::handshake::{ErrorOrigin, ErrorResponse, OkKind, OkResponse};
use crate::model::User;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/auth")
      .service(login)
  );
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "login_type")]
enum LoginForm {
  #[serde(rename = "username")]
  WithUsername(LoginFormUsername),
  #[serde(rename = "email")]
  WithEmail(LoginFormEmail)
}

#[derive(Serialize, Deserialize)]
struct LoginFormUsername {
  username: String,
  password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginFormEmail {
  email: String,
  password: String,
}

#[post("/login")]
async fn login(post: web::Json<LoginForm>, db: Data<Database>) -> HttpResponse {
  let (user, pass) = match &post.0 {
    LoginForm::WithUsername(u) => (User::get(&db.pool, &u.username).await, u.password.as_str()),
    LoginForm::WithEmail(e) => (User::get_email(&db.pool, &e.email).await, e.password.as_str()),
  };
  
  if user.is_none() { return HttpResponse::BadRequest().json(
    ErrorResponse::public_fatal("User does not exist!", ErrorOrigin::Username))}
  let user = user.unwrap();

  if !user.is_verified() {
    return HttpResponse::Unauthorized().json(ErrorResponse::private_fatal("User is not verified!", ErrorOrigin::User));
  }
  
  if !user.verify_password(&db.pool, pass).await {
    return HttpResponse::Unauthorized().json(ErrorResponse::public_fatal("Password is incorrect!", ErrorOrigin::Password));
  }
  
  let auth = user.login(&db.pool).await;
  match auth {
    Err(e) => HttpResponse::InternalServerError().json(
      ErrorResponse::private_fatal(format!("User not logged in: {e}").as_str(), ErrorOrigin::Db)),
    Ok(auth) => {
      let secret = ApiEnv::jwt_secret();
      let jwt = jsonwebtoken::encode(&Header::default(), &auth, &EncodingKey::from_secret(secret.as_bytes()));

      if jwt.is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponse::private_fatal("Could not encode authorization", ErrorOrigin::Auth));
      }
      let jwt = jwt.unwrap();

      OkResponse::new_send("Logged in!", OkKind::Authenticated {
        token: jwt,
        username: user.username().clone()
      })
    }
  }
}

// struct RegisterForm {
//   
// }
// 
// #[post("/register")]
// fn register(post: web::Json<NewPost>, db: Data<Database>) -> HttpResponse {
//   if let Some(_) = match post.0 {
//   LoginForm::WithUsername(u) => User::get(&db.pool, &u.username),
//   LoginForm::WithEmail(e) => User::get_email(&db.pool, &e.email),
//   }.await {
//   return HttpResponse::Conflict().json(ErrorResponse::public_fatal("User already exists!"));
//   }
// }


// #[post("/confirm")]
// fn confirm(post: web::Json<NewPost>, db: Data<Database>) -> HttpResponse {
// }