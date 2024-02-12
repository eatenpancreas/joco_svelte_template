use dotenv::dotenv;
use crate::db;
use bcrypt::DEFAULT_COST;

#[actix_web::test]
async fn init_seed() {
  dotenv().unwrap();
  let pool = db::connect().await.unwrap();

  let _ = sqlx::query!(
    r#"
    INSERT INTO "permission"(name, level)
    VALUES ('owner', 10), ('admin', 8), ('moderator', 3), ('user', 2), ('guest', 0);
    "#,
    )
    .fetch_all(&pool)
    .await;

  let _ = sqlx::query!(
    r#"
    INSERT INTO "user"(username, email, password, is_verified)
    VALUES ('adminohm', 'admin@gmail.com', $1, true);
    "#,
    bcrypt::hash("adminohm", DEFAULT_COST).unwrap()
    )
    .fetch_all(&pool)
    .await;

  let _ = sqlx::query!(
    r#"
    INSERT INTO "user_permission"(user_id, permission_id)
    VALUES ('adminohm', 'owner');
    "#,
    )
    .fetch_all(&pool)
    .await;
}