mod user_permissions;

use actix_web::web;
use crate::middleware::Jwt;

use user_permissions::*;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/user/{username}")
      .wrap(Jwt::default())
      .service(get_user_permissions)
  );
}