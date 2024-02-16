
use serde::Serialize;
use sqlx::PgPool;
use ts_rs::TS;

#[derive(sqlx::FromRow, Serialize, Debug, Clone, TS)]
#[ts(export, export_to = "../src/lib/schema/UserPermission.ts")]
pub struct UserPermission {
  name: String,
  level: i16,
}

impl UserPermission {
  pub fn name(&self) -> &String { &self.name }
  pub fn level(&self) -> i16 { self.level }
  
  #[inline]
  pub async fn from_username(db: &PgPool, username: &str) -> Result<Vec<Self>, sqlx::Error> {
    sqlx::query_as!(
      UserPermission,
      r#"
      SELECT name, level
      FROM "user_permission"
      INNER JOIN "permission" ON "permission".name = "user_permission".permission_id
      WHERE user_id IN (SELECT username FROM "user" WHERE username = $1)
      "#, 
      username
    )
      .fetch_all(db)
      .await
  }
}