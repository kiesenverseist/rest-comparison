use poem::{
    EndpointExt, Route,
    listener::TcpListener,
    middleware::{OpenTelemetryMetrics, Tracing},
};
use poem_openapi::OpenApiService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
use api::Api;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::registry()
        .with(console_subscriber::spawn())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api_service =
        OpenApiService::new(Api, "Hello world", "1.0").server("http://localhost:3000/api");

    let app = Route::new()
        .nest("/api/openapi", api_service.spec_endpoint())
        .nest("/api/doc", api_service.swagger_ui())
        .nest("/api", api_service)
        .with(Tracing)
        .with(OpenTelemetryMetrics::new());

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
