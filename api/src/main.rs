mod routes;
pub mod model;
mod middleware;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod seeders;

use std::io;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use api_lib::db;
use api_lib::env::ApiEnv;
use crate::db::new_db;
use crate::routes::index;

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Connecting...");
    dotenv().unwrap();
    env_logger::init();
    if !ApiEnv::test_all() { return Ok(()); }
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let pool = db::connect().await?;
    
    println!("Running API");
    HttpServer::new(move || {
        App::new()
          .app_data(new_db(pool.clone()))
          .wrap(Cors::permissive().supports_credentials())
          .wrap(Logger::default())
          .service(index)
          .configure(routes::users::config)
          .configure(routes::auth::config)
    }).bind((ApiEnv::api_address(), ApiEnv::api_port()))?
      .run()
      .await
}
