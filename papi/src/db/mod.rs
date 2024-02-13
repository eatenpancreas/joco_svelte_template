
use std::{io, time};
use actix_web::web::Data;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::env::ApiEnv;

pub async fn connect() -> Result<PgPool, io::Error> {
  let pool = PgPoolOptions::new()
    .max_connections(10)
    .max_lifetime(Some(time::Duration::from_secs(100)).unwrap())
    .connect(&ApiEnv::database_url()).await;

  pool.map_err(|err| {
    let error_message = format!("Could not connect to the database! Error: {}", err);
    eprintln!("{}", error_message);
    io::Error::new(io::ErrorKind::NotConnected, error_message)
  })
}

pub struct Database {
  pub pool: PgPool
}

#[inline]
pub fn new_db(pool: PgPool) -> Data<Database> {
  Data::new(Database {
    pool
  })
}