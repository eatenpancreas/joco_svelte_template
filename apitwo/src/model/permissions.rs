
use serde::Serialize;
use sqlx::PgPool;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct UserPermission {
  name: String,
  level: i16,
}

impl UserPermission {
  #[inline]
  pub async fn from_user(db: &PgPool, username: &str) -> Result<Vec<Self>, sqlx::Error> {
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