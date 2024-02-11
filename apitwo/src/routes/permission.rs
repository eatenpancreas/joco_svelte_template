
use actix_web::{web};
use crate::middleware::Jwt;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/permission")
      .wrap(Jwt::default())
  );
}