use std::future::{Ready, ready};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::InternalError;
use actix_web::http::header::HeaderMap;
use actix_web::{Error, HttpMessage, HttpResponse};
use actix_web::web::Data;
use chrono::{DateTime, Utc};
use futures_executor::block_on;
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use crate::db::Database;
use crate::env::ApiEnv;
use crate::handshake::{ErrorOrigin, ErrorResponse};
use crate::model::{User, UserPermission};

type PermissionCheckCallback = fn(Vec<UserPermission>) -> bool;

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtAuth {
  pub(crate) username: String,
  pub(crate) db_sign_moment: DateTime<Utc>,
  
  pub(crate) exp: u64,
  pub(crate) iat: u64,
  pub(crate) nbf: u64
}

pub struct Jwt;

impl Default for Jwt {
  fn default() -> Self {
    Jwt
  }
}

impl<S, B> Transform<S, ServiceRequest> for Jwt
  where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = JwtMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(JwtMiddleware { service }))
  }
}

pub struct JwtMiddleware<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
  where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  // fn call(&self, req: ServiceRequest) -> Self::Future {
  //   if ApiEnv::skip_auth() {
  //     return Box::pin(self.service.call(req));
  //   }
  //   
  //   let db: &Data<Database> = req.app_data().unwrap();
  //   let pool = db.pool.clone();
  //   let headers = req.headers().clone();
  //   let service = &self.service;
  //   let extensions = req.extensions_mut();
  // 
  //   Box::pin(async move {
  //     match authorize(pool, headers).await {
  //       Ok(u) => {
  //         extensions.insert(u);
  //         service.call(req).await
  //       }, Err(e) => Err(e)
  //     }
  //   })
  // }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    if ApiEnv::skip_auth() {
      return Box::pin(self.service.call(req));
    }

    // let authorized = {
    //   let db: &Data<Database> = req.app_data().unwrap();
    //   let pool = db.pool.clone();
    //   let headers = req.headers().clone();
    //   let mut extensions = req.extensions_mut();
    // 
    //   match block_on(authorize(pool, headers)) {
    //     Ok(u) => {
    //       extensions.insert(u);
    //       Ok(())
    //     }, Err(e) => Err(e)
    //   }
    // };
    // 
    // if let Err(e) = authorized {
    //   return Box::pin(async move { Err(e) });
    // }

    Box::pin(self.service.call(req))
    // Box::pin(async move {
    //   self.service.call(req).await
      // match authorized {
      //   Ok(_) => self.service.call(req).await, Err(e) => Err(e)
      // }
    // })
  }
}

async fn authorize(
  pool: Pool<Postgres>, headers: HeaderMap
) -> Result<AuthorizedUser, Error> {
  let auth_header = headers.get("Authorization").ok_or(unauthorized("No Authorization header present!", ErrorOrigin::Auth))?
    .to_str().map_err(|_| unauthorized("Could not decode Authorization header!", ErrorOrigin::Auth))?;
  
  if !(auth_header.starts_with("Bearer ")) { return Err(unauthorized("Authorisation requires Bearer token!", ErrorOrigin::Auth)) }
  
  let token = auth_header[7..].to_string();
  let secret = ApiEnv::jwt_secret();
  let key = DecodingKey::from_secret(secret.as_bytes());
  let validation = Validation::new(Algorithm::HS256);
  let decoding = decode::<JwtAuth>(&*token, &key, &validation).map_err(
    |e| unauthorized(format!("Token is invalid! {e}").as_str(), ErrorOrigin::Auth))?;
  
  let user = User::get(&pool, &decoding.claims.username).await
    .ok_or(unauthorized("Could not find user!", ErrorOrigin::User))?;
  
  if !user.is_verified() {
    return Err(unauthorized("User is not verified!", ErrorOrigin::User));
  }

  if user.last_login().timestamp() != decoding.claims.db_sign_moment.timestamp() {
    return Err(unauthorized("Token is old!", ErrorOrigin::Auth));
  }
  
  let perms = UserPermission::from_username(&pool, user.username().clone().as_str()).await;
  if perms.is_err() {
    return Err(unauthorized("Could not get permissions!", ErrorOrigin::Perms));
  }
  
  // send user with permissions to request
  Ok(AuthorizedUser {
    u: user,
    permissions: perms.unwrap()
  })
}

#[derive(Clone)]
pub struct AuthorizedUser {
  u: User,
  permissions: Vec<UserPermission>
}

impl AuthorizedUser {
  pub fn is_username(&self, username: &str) -> bool {
    self.u.username() == username
  }
  pub fn has_level(&self, level: i16) -> bool {
    self.permissions.iter().any(|perm| perm.level() >= level)
  }
  
  pub fn has_permission(&self, permission: &str) -> bool {
    self.permissions.iter().any(|perm| perm.name() == permission)
  }
}


fn unauthorized(message: &str, on: ErrorOrigin) -> Error {
  InternalError::from_response("UNAUTHORIZED", HttpResponse::Unauthorized()
    .json(ErrorResponse::private_fatal(message, on))).into()
}