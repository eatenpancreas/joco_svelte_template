use std::borrow::Cow;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/lib/handshake/ext/ValidationError.ts")]
pub struct ValidationError {
  pub code: Cow<'static, str>,
  pub message: Option<Cow<'static, str>>,
  pub params: HashMap<Cow<'static, str>, String>,
}

impl From<validator::ValidationError> for ValidationError {
  fn from(value: validator::ValidationError) -> Self {
    ValidationError {
      code: value.code,
      message: value.message,
      params: value.params.iter().map(|(x, v)| (x.clone(), v.to_string())).into_iter().collect()
    }
  }
}