use sqlx::{PgPool};

pub struct User;

struct Password {
  password: String
}

impl User {
  pub async fn exists(db: &PgPool, username: &String) -> bool {
    let pass = sqlx::query_as!(Password, 
      r#"SELECT username FROM public."User" WHERE "User".username = $1"#, 
      username).fetch_one(db).await;

    pass.is_ok()
  }
}