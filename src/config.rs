use serde::Deserialize;

use std::{env, fs, net::SocketAddr};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub listen: Listen,
    pub sqlserver: SqlServer,
    pub database: DataBase,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Listen {
    address: String,
    port: u16,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SqlServer {
    pub connection_type: ConnectionType,
    pub address: String,
    pub port: u16,
    pub pool_size: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DataBase {
    pub user: String,
    pub passwd: String,
    pub db: String,
}
#[derive(Debug, Copy, Clone, Deserialize, PartialEq)]
pub enum ConnectionType {
    PosgreSQL,
    MariaDB,
    MySQL,
}

impl Config {
    pub fn init() -> Config {
        let args: Vec<String> = env::args().collect();

        let path = if args.len() > 1 {
            args[2].as_str()
        } else {
            "Config.toml"
        };

        let file = fs::read_to_string(path).unwrap_or(String::from(""));
        let config: Config = toml::from_str(file.as_str()).unwrap();

        if config.sqlserver.connection_type != ConnectionType::PosgreSQL {
            panic!("Only connection type available for the momment PosgreSQL");
        }
        config
    }
}

impl Listen {
    pub fn socketaddress(&self) -> SocketAddr {
        let address = format!("{}:{}", self.address, self.port);
        let sa: SocketAddr = address
            .as_str()
            .parse()
            .expect("Unable to parse socket address");
        sa
    }
}
