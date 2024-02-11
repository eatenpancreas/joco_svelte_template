
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
  errors: Vec<Error>
}

impl ErrorResponse {
  pub fn private_fatal(message: &str) -> Self {
    ErrorResponse {
      errors: vec![Error {
        kind: ErrorKind::PrivateFatal,
        message: message.to_string()
      }]
    }
  }
  pub fn public_fatal(message: &str) -> Self {
    ErrorResponse {
      errors: vec![Error {
        kind: ErrorKind::PublicFatal,
        message: message.to_string()
      }]
    }
  }
}

#[derive(Serialize)]
pub enum ErrorKind {
  PublicMinor,
  PublicFatal,
  PrivateMinor,
  PrivateFatal
}

#[derive(Serialize)]
pub struct Error {
  kind: ErrorKind,
  message: String
}