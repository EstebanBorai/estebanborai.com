use std::env;
use std::str::FromStr;

pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub server_config: rocket::Config,
}

impl Config {
    pub fn new() -> Self {
        let port = Config::env_var::<u16>("PORT");
        let database_url = Config::env_var::<String>("DATABASE_URL");
        let server_config = rocket::Config {
            port,
            ..rocket::Config::default()
        };

        Config {
            database_url,
            port,
            server_config,
        }
    }

    fn env_var<T: FromStr>(key: &str) -> T {
        let value = env::var(key).expect(&format!("Missing environment variable: {}", key));

        if let Ok(parsed) = str::parse::<T>(&value) {
            return parsed;
        }

        panic!(
            "Failed to parse environment variable value from key: {}",
            key
        );
    }
}
