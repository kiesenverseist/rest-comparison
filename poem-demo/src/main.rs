use poem::{
    EndpointExt, Route,
    listener::TcpListener,
    middleware::{OpenTelemetryMetrics, Tracing},
};
use poem_openapi::OpenApiService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use color_eyre::{Result, eyre::Context};

mod api;
use api::Api;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(console_subscriber::spawn())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").wrap_err("DATABASE_URL is not set")?;
    let pool = sqlx::PgPool::connect(&db_url)
        .await
        .wrap_err(format!("Could not connect to db at {db_url}"))?;
    dbg!("Connected to database");

    let api_service =
        OpenApiService::new(Api, "Hello world", "1.0").server("http://localhost:3000/api");

    let app = Route::new()
        .nest("/api/openapi", api_service.spec_endpoint())
        .nest("/api/doc", api_service.swagger_ui())
        .nest("/api", api_service)
        .with(Tracing)
        .with(OpenTelemetryMetrics::new())
        .data(pool);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
        .wrap_err("Problem starting server")
}
