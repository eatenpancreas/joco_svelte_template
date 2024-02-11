use std::env;

pub struct ApiEnv;

impl ApiEnv {
  pub fn skip_auth() -> bool { Self::_skip_auth().unwrap() }
  fn _skip_auth() -> Result<bool, String> {
    let skip_auth_str = env::var("SKIP_AUTH").map_err(|x| x.to_string())?;
    skip_auth_str.parse::<bool>().map_err(|x| x.to_string())
  }
  pub fn database_url() -> String { Self::_database_url().unwrap() }
  fn _database_url() -> Result<String, String> {
    env::var("DATABASE_URL").map_err(|x| x.to_string())
  }
  pub fn jwt_secret() -> String { Self::_jwt_secret().unwrap() }
  fn _jwt_secret() -> Result<String, String> {
    env::var("JWT_SECRET").map_err(|x| x.to_string())
  }
  
  pub fn test_all() -> bool {
    log(Self::_database_url(), "DATABASE_URL") &&
    log(Self::_skip_auth(), "SKIP_AUTH") &&
    log(Self::_jwt_secret(), "JWT_SECRET")
  }
}

fn log<T>(env: Result<T, String>, name: &str) -> bool {
  match env { 
    Ok(_) => true,
    Err(e) => {
      eprintln!(".env {} Error: {}", name, e);
      false
    }
  }
}