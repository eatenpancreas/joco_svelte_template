use std::env;

pub struct ApiEnv;

impl ApiEnv {
  pub fn skip_auth() -> bool { Self::_skip_auth().unwrap() }
  fn _skip_auth() -> Result<bool, String> {
    let var = env::var("SKIP_AUTH").map_err(|x| x.to_string())?;
    var.parse::<bool>().map_err(|x| x.to_string())
  }
  pub fn email_auth_enabled() -> bool { Self::_email_auth_enabled().unwrap() }
  fn _email_auth_enabled() -> Result<bool, String> {
    let var = env::var("EMAIL_AUTH_ENABLED").map_err(|x| x.to_string())?;
    var.parse::<bool>().map_err(|x| x.to_string())
  }
  pub fn database_url() -> String { Self::_database_url().unwrap() }
  fn _database_url() -> Result<String, String> {
    env::var("DATABASE_URL").map_err(|x| x.to_string())
  }

  pub fn api_address() -> String { Self::_api_address().unwrap() }
  fn _api_address() -> Result<String, String> {
    env::var("API_ADDRESS").map_err(|x| x.to_string())
  }

  pub fn api_port() -> u16 { Self::_api_port().unwrap() }
  fn _api_port() -> Result<u16, String> {
    let var = env::var("API_PORT").map_err(|x| x.to_string())?;
    var.parse::<u16>().map_err(|x| x.to_string())
  }
  
  pub fn jwt_secret() -> String { Self::_jwt_secret().unwrap() }
  fn _jwt_secret() -> Result<String, String> {
    env::var("JWT_SECRET").map_err(|x| x.to_string())
  }
  
  pub fn test_all() -> bool {
    log(Self::_database_url(), "DATABASE_URL") &&
    log(Self::_skip_auth(), "SKIP_AUTH") &&
    log(Self::_jwt_secret(), "JWT_SECRET") &&
    log(Self::_email_auth_enabled(), "EMAIL_AUTH_ENABLED") &&
    log(Self::_api_address(), "API_ADDRESS") &&
    log(Self::_api_port(), "API_PORT")
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