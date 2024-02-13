use actix_web::{HttpResponse};
use serde::{Deserialize, Serialize};
use crate::model::{User, UserPermission};

#[derive(Serialize)]
pub struct ErrorResponse {
  message: String,
  errors: Vec<Error>
}

impl ErrorResponse {
  pub fn private_fatal(message: &str, on: ErrorOrigin) -> Self {
    ErrorResponse {
      message: message.to_string(),
      errors: vec![Error {
        kind: ErrorKind::PrivateFatal,
        message: message.to_string(),
        origin: on
      }]
    }
  }
  pub fn public_fatal(message: &str, on: ErrorOrigin) -> Self {
    ErrorResponse {
      message: message.to_string(),
      errors: vec![Error {
        kind: ErrorKind::PublicFatal,
        message: message.to_string(),
        origin: on
      }]
    }
  }
}

#[derive(Serialize)]
pub enum ErrorOrigin {
  #[serde(rename = "username")]
  Username,
  #[serde(rename = "user")]
  User,
  #[serde(rename = "password")]
  Password,
  #[serde(rename = "db")]
  Db,
  #[serde(rename = "authentication")]
  Auth,
  #[serde(rename = "permissions")]
  Perms,
}

#[derive(Serialize)]
#[serde(tag = "ok_kind", content = "data")]
pub enum OkKind<T: Serialize> {
  #[serde(rename = "simple")]
  Simple,
  #[serde(rename = "authenticated")]
  Authenticated {
    token: String,
    username: String
  },
  #[serde(rename = "redirected")]
  Redirected {
    to: String
  },
  #[serde(rename = "user_permissions")]
  UserPermissions {
    username: String,
    permissions: Vec<UserPermission>
  },
  #[serde(rename = "list")]
  List(Vec<T>),
  #[serde(rename = "item")]
  Item(T),
}

#[derive(Serialize)]
pub enum ErrorKind {
  #[serde(rename = "public_minor")]
  PublicMinor,
  #[serde(rename = "public_fatal")]
  PublicFatal,
  #[serde(rename = "private_minor")]
  PrivateMinor,
  #[serde(rename = "private_fatal")]
  PrivateFatal
}

#[derive(Serialize)]
pub struct Error {
  kind: ErrorKind,
  message: String,
  origin: ErrorOrigin,
}

#[derive(Serialize)]
pub struct OkResponse<T: Serialize> {
  message: String,
  pub errors: Vec<Error>,

  #[serde(flatten)]
  ok_kind: OkKind<T>
}

impl<T: Serialize> OkResponse<T> {
  pub fn new_send<S: ToString>(message: S, kind: OkKind<T>) -> HttpResponse {
    HttpResponse::Ok().json(OkResponse {
      errors: vec![],
      ok_kind: kind,
      message: message.to_string(),
    })
  }
}


#[derive(Deserialize, Serialize)]
pub struct DbRange {
  pub limit: Option<u16>,
  pub offset: Option<u16>
}