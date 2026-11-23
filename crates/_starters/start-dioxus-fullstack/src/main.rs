#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]

mod app;
mod components;
mod domain;
mod utils;

use app::App;

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        use dioxus::server::axum::Extension;
        use sqlx::ConnectOptions;
        use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let connect_options = database_url
            .parse::<PgConnectOptions>()
            .expect("Invalid DATABASE_URL")
            .log_statements(log::LevelFilter::Off)
            .log_slow_statements(log::LevelFilter::Warn, std::time::Duration::from_millis(100));

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(connect_options)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

        Ok(dioxus::server::router(App).layer(Extension(pool)))
    });
}
