use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

pub struct Config {
    pub database_url: String,
    pub github_api_user: String,
    pub github_api_token: String,
    pub port: u16,
    pub server_config: rocket::Config,
}

impl Config {
    pub fn new() -> Self {
        let port = Config::env_var::<u16>("PORT");
        let database_url = Config::env_var::<String>("DATABASE_URL");
        let github_api_user = Config::env_var::<String>("GITHUB_API_USER");
        let github_api_token = Config::env_var::<String>("GITHUB_API_TOKEN");
        let address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let server_config = rocket::Config {
            address,
            port,
            ..rocket::Config::default()
        };

        Config {
            database_url,
            github_api_user,
            github_api_token,
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
