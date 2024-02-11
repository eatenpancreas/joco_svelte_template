mod routes;
pub mod db;
pub mod model;
mod middleware;
mod env;
mod errors;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod seeders;

use std::io;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use crate::db::new_db;
use crate::env::ApiEnv;
use crate::routes::index;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().unwrap();
    env_logger::init();
    if !ApiEnv::test_all() { return Ok(()); }
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let pool = db::connect().await?;
    
    sqlx::migrate!()
      .run(&pool)
      .await.unwrap();
    
    HttpServer::new(move || {
        App::new()
          .app_data(new_db(pool.clone()))
          .wrap(Logger::default())
          .service(index)
          .configure(routes::permission::config)
          .configure(routes::user::config)
    }).bind(("127.0.0.1", 8080))?
      .run()
      .await
}
