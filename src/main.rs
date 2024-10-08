use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Read from configuration.yaml file
    let config = get_configuration().expect("Failed to read config file");
    let connection_pool = PgPool::connect_lazy(&config.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres");
    // Use localhost and stamped in config port
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
