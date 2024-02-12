
use crate::middleware::Jwt;
use actix_web::{get, HttpResponse, web};
use actix_web::web::Data;
use crate::db::Database;
use crate::handshake::{ErrorOrigin, ErrorResponse, OkKind, OkResponse};
use crate::model::UserPermission;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/users/{username}")
      .wrap(Jwt::default())
      .service(get_user_permissions)
  );
}

#[get("/permissions")]
async fn get_user_permissions(username: web::Path<String>, db: Data<Database>) -> HttpResponse {
  match UserPermission::from_user(&db.pool, &username).await {
    Ok(perms) => {
      OkResponse::new_send(format!("Showing permissions for {username}"), OkKind::UserPermissions {
        username: username.to_string(),
        permissions: perms
      })
    }
    Err(_) => {
      HttpResponse::InternalServerError().json(ErrorResponse::public_fatal("Could not get permissions!", ErrorOrigin::Perms))
    }
  }
}