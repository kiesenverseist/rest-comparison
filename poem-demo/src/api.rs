use std::time::SystemTime;

use poem::{error::InternalServerError, web::Data, Result};
use poem_openapi::{
    Object, OpenApi,
    param::Query,
    payload::{Json, PlainText},
};
use sqlx::PgPool;

#[derive(Object)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

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

    /// Create a new todo
    #[oai(path = "/todo", method = "post")]
    async fn create_todo(
        &self,
        pool: Data<&PgPool>,
        description: PlainText<String>,
    ) -> Result<Json<i32>> {

        let id = sqlx::query!("insert into todos (title) values ($1) returning id", description.0)
            .fetch_one(pool.0)
            .await
            .map_err(InternalServerError)?
            .id;

        Ok(Json(id))
    }
}
