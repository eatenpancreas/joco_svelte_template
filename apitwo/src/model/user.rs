use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::get_current_timestamp;
use sqlx::{PgPool};
use crate::middleware::JwtAuth;

pub struct User {
  username: String,
  email: String,
  is_verified: bool,
  last_login: NaiveDateTime
}

struct Password {
  password: String,
}

impl User {
  pub fn username(&self) -> &String { &self.username }
  pub fn last_login(&self) -> &NaiveDateTime { &self.last_login }
  pub fn is_verified(&self) -> bool { self.is_verified }
  
  pub async fn get(db: &PgPool, username: &String) -> Option<Self> {
    sqlx::query_as!(
      Self,
      r#"SELECT username, email, is_verified, last_login FROM "user" WHERE "user".username = $1"#, 
      username
    ).fetch_one(db).await.ok()
  }
  
  pub async fn verify_password(&self, db: &PgPool, password: &str) -> bool {
    let db_pass = sqlx::query_as!(
      Password,
      r#"SELECT password FROM "user" WHERE "user".username = $1"#, 
      self.username
    ).fetch_one(db).await;
    
    if db_pass.is_err() { return false }
    let db_pass = db_pass.unwrap().password;
    
    bcrypt::verify(password, db_pass.as_str()).unwrap_or(false)
  }

  pub async fn get_email(db: &PgPool, email: &String) -> Option<Self> {
    sqlx::query_as!(
      Self,
      r#"SELECT username, email, is_verified, last_login FROM "user" WHERE "user".email = $1"#, 
      email
    ).fetch_one(db).await.ok()
  }
  
  pub async fn login(&self, db: &PgPool) -> Result<JwtAuth, sqlx::Error> {
    let now = Utc::now();
    if let Err(e) = sqlx::query!(
      r#"UPDATE "user" SET last_login=$1 WHERE "user".username = $2"#, 
      now.naive_utc(),
      &self.username
    ).execute(db).await {
      return Err(e)
    }
    
    let duration = Duration::hours(10).num_seconds() as u64;

    Ok(JwtAuth {
      exp: get_current_timestamp() + duration,
      iat: get_current_timestamp(),
      nbf: get_current_timestamp(),
      username: self.username.clone(),
      db_sign_moment: now,
    })
  }
}