
use crate::middleware::{AuthorizedUser, Jwt};
use actix_web::{get, HttpResponse, web};
use actix_web::web::{Data, ReqData};
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
async fn get_user_permissions(username: web::Path<String>, db: Data<Database>, au: ReqData<AuthorizedUser>) -> HttpResponse {
  let au = au.into_inner();
  if !au.has_level(8) && !au.is_username(&username) {
    return HttpResponse::Unauthorized().json(ErrorResponse::public_fatal("Not authorized!", ErrorOrigin::Auth));
  }
  
  match UserPermission::from_username(&db.pool, &username).await {
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