
use crate::middleware::{AuthorizedUser, Jwt};
use actix_web::{delete, get, HttpResponse, web};
use actix_web::web::{Data, Query, ReqData};
use serde::Serialize;
use ts_rs::TS;
use crate::db::Database;
use crate::handshake::{DbRange, ErrorOrigin, ErrorResponse, OkResponse, OkResponseKind};
use crate::model::{User, UserPermission};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/users")
      .wrap(Jwt::default())
      .service(get_all)
      .service(
        web::scope("/{username}")
          .service(get)
          .service(delete)
          .service(get_user_permissions)
      )
  );
}


#[get("/")]
async fn get_all(range: Query<DbRange>, db: Data<Database>, au: ReqData<AuthorizedUser>) -> HttpResponse {
  let au = au.into_inner();
  if !au.has_level(8) {
    return HttpResponse::Unauthorized().json(ErrorResponse::public_fatal("Not authorized!", ErrorOrigin::Auth));
  }

  if let Some(u) = User::get_all(&db.pool, range.limit.unwrap_or(40) as i64, range.offset.unwrap_or(0) as i64).await {
    return OkResponse::new_send("Users found!", OkResponseKind::Data(u));
  }

  HttpResponse::InternalServerError().json(ErrorResponse::public_fatal("Could not get users!", ErrorOrigin::Db))
}

#[get("/")]
async fn get(username: web::Path<String>, db: Data<Database>, au: ReqData<AuthorizedUser>) -> HttpResponse {
  let au = au.into_inner();
  if !au.has_level(8) && !au.is_username(&username) {
    return HttpResponse::Unauthorized().json(ErrorResponse::public_fatal("Not authorized!", ErrorOrigin::Auth));
  }

  if let Some(u) = User::get(&db.pool, username.as_str()).await {
    return OkResponse::new_send("User found!", OkResponseKind::Data(u));
  }

  HttpResponse::InternalServerError().json(ErrorResponse::public_fatal("Could not get users!", ErrorOrigin::Db))
}

#[delete("/")]
async fn delete(username: web::Path<String>, db: Data<Database>, au: ReqData<AuthorizedUser>) -> HttpResponse {
  let au = au.into_inner();
  if !au.has_level(8) && !au.is_username(&username) {
    return HttpResponse::Unauthorized().json(ErrorResponse::public_fatal("Not authorized!", ErrorOrigin::Auth));
  }
  
  if let Some(u) = User::get(&db.pool, &username).await {
    if let Ok(_) = u.delete(&db.pool).await {
      return OkResponse::new_send("Deleted user!", OkResponseKind::<()>::Simple)
    }
  }

  HttpResponse::InternalServerError().json(ErrorResponse::public_fatal("Could not delete user!", ErrorOrigin::Db))
}

#[derive(sqlx::FromRow, Serialize, Debug, Clone, TS)]
#[ts(export, export_to = "../src/lib/schema/UserPermissions.ts")]
struct UserPermissions {
  username: String,
  permissions: Vec<UserPermission>
}

#[get("/permissions")]
async fn get_user_permissions(username: web::Path<String>, db: Data<Database>, au: ReqData<AuthorizedUser>) -> HttpResponse {
  let au = au.into_inner();
  if !au.has_level(8) && !au.is_username(&username) {
    return HttpResponse::Unauthorized().json(ErrorResponse::public_fatal("Not authorized!", ErrorOrigin::Auth));
  }
  
  match UserPermission::from_username(&db.pool, &username).await {
    Ok(perms) => {
      OkResponse::new_send(format!("Showing permissions for {username}"), OkResponseKind::Data( UserPermissions{
        username: username.to_string(),
        permissions: perms
      }))
    }
    Err(_) => {
      HttpResponse::InternalServerError().json(ErrorResponse::public_fatal("Could not get permissions!", ErrorOrigin::Perms))
    }
  }
}