use std::time::SystemTime;

use poem::{listener::TcpListener, middleware::{OpenTelemetryMetrics, Tracing}, EndpointExt, Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct Api;

#[OpenApi]
impl Api {
   
    /// Short summary / title
    ///
    /// This is the longer description
    ///
    /// * `name` - The optional name to include in the response
    #[oai(path = "/hello", method = "get")]
    async fn hello(&self, name: Query<Option<String>>) -> PlainText<String>{
        match name.0 {
            Some(name) => PlainText(format!("hello {name}")),
            None => PlainText("hello".to_string()),
        }
    }
    
    /// The time
    ///
    /// return the current time
    #[oai(path = "/time", method = "get")]
    async fn time(&self) -> PlainText<String>{
        PlainText(format!("{:#?}", SystemTime::now()))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    tracing_subscriber::registry()
        .with(console_subscriber::spawn())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api_service = OpenApiService::new(Api, "Hello world", "1.0")
        .server("http://localhost:3000/api");

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
