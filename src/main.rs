#![allow(unused_imports)]
use env_logger::Env;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPool;
use std::net::TcpListener;
// use tracing::dispatcher::set_global_default;
use zero2prod::configuration::{self, get_configuration};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

// use tracing::{subscriber::set_global_default, Subscriber};
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// use tracing_log::LogTracer;
// use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

type Result<T> = std::result::Result<T, std::io::Error>; // Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set.
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let subscriber = get_subscriber("rust_zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");

    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    // Run application
    run(listener, connection_pool)?.await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use zero2prod::routes::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success());
    }
}
