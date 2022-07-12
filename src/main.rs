use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".to_string(), "info".to_string(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(10))
        .connect_lazy_with(configuration.database.with_db())
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}

