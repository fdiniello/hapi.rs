mod config;
mod postgres_pool;
mod service;

use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::init();

    let listen_addr = config.listen.socketaddress();

    let sql_pool = postgres_pool::new(config.sql_connection(), config.sqlserver.pool_size);

    service::run(listen_addr, sql_pool).await;
    
}
