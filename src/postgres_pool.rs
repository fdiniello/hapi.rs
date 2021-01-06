extern crate r2d2_postgres;
use crate::config::Config;
use crate::config::ConnectionType;

use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

impl Config {
    pub fn sql_connection(&self) -> String {
        let mut conn = String::new();
        match self.sqlserver.connection_type {
            ConnectionType::PosgreSQL => conn.push_str("postgresql"),
            ConnectionType::MariaDB => conn.push_str("mariadb"),
            ConnectionType::MySQL => conn.push_str("mysql"),
        }

        let usr = if self.database.passwd.is_empty() {
            format!("{}", self.database.user)
        } else {
            format!("{}:{}", self.database.user, self.database.passwd)
        };

        let url = format!("{}:{}", self.sqlserver.address, self.sqlserver.port);
        let database = self.database.db.clone();

        format!(
            "{conn}://{usr}@{url}/{db}",
            conn = conn,
            usr = usr,
            url = url,
            db = database
        )
    }
}

pub fn new(url: String, size: i32) -> r2d2::Pool<PostgresConnectionManager<NoTls>> {
    let manager = PostgresConnectionManager::new(url.parse().unwrap(), NoTls);

    r2d2::Pool::builder()
        .max_size(size as u32)
        .build(manager)
        .unwrap()
}
