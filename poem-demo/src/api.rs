use std::time::SystemTime;

use poem_openapi::{OpenApi, param::Query, payload::PlainText};

pub struct Api;

#[OpenApi]
impl Api {
    /// Short summary / title
    ///
    /// This is the longer description
    ///
    /// * `name` - The optional name to include in the response
    #[oai(path = "/hello", method = "get")]
    async fn hello(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello {name}")),
            None => PlainText("hello".to_string()),
        }
    }

    /// The time
    ///
    /// return the current time
    #[oai(path = "/time", method = "get")]
    async fn time(&self) -> PlainText<String> {
        PlainText(format!("{:#?}", SystemTime::now()))
    }
}
