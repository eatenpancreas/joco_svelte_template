mod handshake_ext;

use std::collections::HashMap;
use actix_web::{HttpResponse};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::{Validate, ValidationErrors};
use serde_with::serde_as;
use crate::handshake::handshake_ext::ValidationError;

#[derive(Serialize, TS)]
#[ts(export, export_to = "../src/lib/handshake/ErrorResponse.ts")]
pub struct ErrorResponse {
  message: String,
  kind: ErrorResponseKind
}


#[derive(Serialize, TS)]
#[ts(export, export_to = "../src/lib/handshake/ErrorResponseKind.ts")]
#[serde(tag = "err_kind", content = "response")]
pub enum ErrorResponseKind {
  #[serde(rename = "list")]
  Vec(Vec<Error>),
  #[serde(rename = "single")]
  Single(Error),
  #[serde(rename = "validation")]
  Validation(Validation)
}

impl ErrorResponse {
  pub fn private_fatal(message: &str, on: ErrorOrigin) -> Self {
    ErrorResponse {
      message: message.to_string(),
      kind: ErrorResponseKind::Single(Error {
        kind: ErrorKind::PrivateFatal,
        message: message.to_string(),
        origin: on
      })
    }
  }
  pub fn public_fatal(message: &str, on: ErrorOrigin) -> Self {
    ErrorResponse {
      message: message.to_string(),
      kind: ErrorResponseKind::Single(Error {
        kind: ErrorKind::PublicFatal,
        message: message.to_string(),
        origin: on
      })
    }
  }
  
  pub fn validation(message: &str, on: ValidationErrors) -> Self {
    let fields = on.field_errors().iter()
      .map(|(s, err)| (s.to_string(), err.iter().map(|err| ValidationError::from(err.clone())).collect() )).collect();
    
    ErrorResponse {
      message: message.to_string(),
      kind: ErrorResponseKind::Validation(Validation { fields })
    }
  }
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../src/lib/handshake/ErrorOrigin.ts")]
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
  #[serde(rename = "fetch")]
  Fetch,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../src/lib/handshake/OkResponseKind.ts")]
#[serde(tag = "ok_kind", content = "response")]
pub enum OkResponseKind<T: Serialize + TS> {
  #[serde(rename = "simple")]
  Simple,
  #[serde(rename = "redirected")]
  Redirected {
    to: String
  },
  #[serde(rename = "data")]
  Data(T),
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../src/lib/handshake/ErrorKind.ts")]
pub enum ErrorKind {
  #[serde(rename = "public_minor")]
  PublicMinor,
  #[serde(rename = "public_fatal")]
  PublicFatal,
  #[serde(rename = "private_minor")]
  PrivateMinor,
  #[serde(rename = "private_fatal")]
  PrivateFatal,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "../src/lib/handshake/Validation.ts")]
struct Validation {
  fields: HashMap<String, Vec<ValidationError>>,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../src/lib/handshake/Error.ts")]
pub struct Error {
  kind: ErrorKind,
  message: String,
  origin: ErrorOrigin,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../src/lib/handshake/OkResponse.ts")]
pub struct OkResponse<T: Serialize + TS> {
  message: String,
  pub errors: Vec<Error>,
  ok: OkResponseKind<T>
}

impl<T: Serialize + TS> OkResponse<T> {
  pub fn new_send<S: ToString>(message: S, kind: OkResponseKind<T>) -> HttpResponse {
    HttpResponse::Ok().json(OkResponse {
      errors: vec![],
      ok: kind,
      message: message.to_string(),
    })
  }
}

#[derive(Deserialize, Serialize, TS, Validate)]
#[ts(export, export_to = "../src/lib/handshake/DbRange.ts")]
pub struct DbRange {
  #[validate(range(min = 1, max = 40))]
  pub limit: Option<u16>,
  #[validate(range(min = 0))]
  pub offset: Option<u16>
}