use poem::web::Data;
use poem_openapi::{payload::PlainText, OpenApi};

use crate::data::SharedData;

pub struct HelloWorldEndpoint;

#[OpenApi]
impl HelloWorldEndpoint {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self, data: Data<&SharedData>) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}
