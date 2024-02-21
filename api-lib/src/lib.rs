pub mod handshake;
pub mod env;
use dotenv::dotenv;
pub mod db;

#[actix_web::test]
async fn run_migration() {
    dotenv().unwrap();
    let pool = db::connect().await.unwrap();

    if let Err(e) = sqlx::migrate!("../migrations")
        .run(&pool)
        .await {
        println!("-- Migration issue | {e} --")
    }
}
