use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_config, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_config().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(address)?;

    run(listener, connection)?.await
}
