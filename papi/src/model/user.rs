use std::io;
use std::io::ErrorKind;
use bcrypt::DEFAULT_COST;
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::get_current_timestamp;
use serde::Serialize;
use sqlx::{PgPool};
use ts_rs::TS;
use crate::middleware::JwtAuth;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, export_to = "../src/lib/schema/User.ts")]
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
  
  pub async fn get(db: &PgPool, username: &str) -> Option<Self> {
    sqlx::query_as!(
      Self,
      r#"SELECT username, email, is_verified, last_login FROM "user" WHERE "user".username = $1"#, 
      username
    ).fetch_one(db).await.ok()
  }
  
  pub async fn get_all(db: &PgPool, limit: i64, offset: i64) -> Option<Vec<Self>> {
    sqlx::query_as!(
      Self,
      r#"SELECT username, email, is_verified, last_login FROM "user" LIMIT $1 OFFSET $2"#, 
      limit, offset
    ).fetch_all(db).await.ok()
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
  
  pub async fn register(db: &PgPool, username: &str, email: &str, password: &str) -> Result<Self, sqlx::Error> {
    let hash = bcrypt::hash(password, DEFAULT_COST);
    
    if let Err(_) = hash {
      return Err(sqlx::Error::Io(io::Error::new(ErrorKind::InvalidData, "Could not hash password!")));
    }
    
    let hash = hash.unwrap();
    
    if let Err(e) = sqlx::query!(
      r#"INSERT INTO "user" (username, email, password, is_verified) VALUES ($1, $2, $3, false); "#, 
      username, email, hash
    ).execute(db).await {
      return Err(e)
    }

    if let Err(e) = sqlx::query!(
      r#"INSERT INTO "user_permission" (user_id, permission_id) VALUES ($1, 'guest'); "#, 
      username
    ).execute(db).await {
      return Err(e)
    }
    
    Self::get(db, username).await.ok_or(sqlx::Error::ColumnNotFound("User".to_string()))
  }
  
  pub async fn set_verified(&mut self, db: &PgPool, is_verified: bool) -> Result<(), sqlx::Error> {
    self.is_verified = true;
    
    if let Err(e) = sqlx::query!(
      r#"UPDATE "user" SET is_verified=$1 WHERE "user".username = $2"#, 
      is_verified,
      &self.username
    ).execute(db).await {
      return Err(e)
    }
    Ok(())
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
  
  pub async fn delete(self, db: &PgPool) -> Result<(), sqlx::Error> {
    if let Err(e) = sqlx::query!(
      r#"DELETE FROM "user_permission" WHERE "user_permission".user_id = $1"#, 
      &self.username
    ).execute(db).await {
      return Err(e)
    }
    if let Err(e) = sqlx::query!(
      r#"DELETE FROM "user" WHERE "user".username = $1"#, 
      &self.username
    ).execute(db).await {
      return Err(e)
    }
    Ok(())
  }
}