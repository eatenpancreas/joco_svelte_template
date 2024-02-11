use actix_web::{get, HttpResponse, web};
use actix_web::web::Data;
use serde::Serialize;
use crate::db::Database;
use crate::errors::ErrorResponse;
use crate::model::UserPermission;

#[derive(Serialize)]
struct UserPermissions {
  username: String,
  permissions: Vec<UserPermission>
}

#[get("/permissions")]
async fn get_user_permissions(username: web::Path<String>, db: Data<Database>) -> HttpResponse {
  match UserPermission::from_user(&db.pool, &username).await {
    Ok(perms) => {
      HttpResponse::Ok().json(UserPermissions {
        username: username.to_string(),
        permissions: perms
      })
    }
    Err(_) => {
      HttpResponse::InternalServerError().json(ErrorResponse::public_fatal("Could not get permissions!"))
    }
  }
}