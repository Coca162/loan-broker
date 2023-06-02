use poem_openapi::{payload::PlainText, OpenApi};

pub struct HelloWorldEndpoint;

#[OpenApi]
impl HelloWorldEndpoint {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World!")
    }
}
