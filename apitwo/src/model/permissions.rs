
use serde::Serialize;
use sqlx::PgPool;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct UserPermission {
  permission: String,
  level: i16,
}

impl UserPermission {
  #[inline]
  pub async fn from_user(db: &PgPool, username: &str) -> Result<Vec<Self>, sqlx::Error> {
    sqlx::query_as!(
    UserPermission,
    r#"
    SELECT permission, level
    FROM public."UserPermission"
    INNER JOIN public."Permission" ON "Permission".id = "UserPermission".permission_id
    WHERE user_id IN (SELECT id FROM "User" WHERE username = $1)
    "#, 
    username
    )
      .fetch_all(db)
      .await
  }
}