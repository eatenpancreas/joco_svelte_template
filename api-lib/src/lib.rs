pub mod handshake;
pub mod env;
pub mod db;

use dotenv::dotenv;

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