use std::{
    env::{self, VarError},
    sync::Arc,
};

use data::{EnvConfig, SharedData};
use dotenvy::dotenv;
use poem::{
    get,
    listener::TcpListener,
    session::{CookieConfig, MemoryStorage, ServerSession},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use routes::hello_world::HelloWorldEndpoint;

use sqlx::postgres::PgPoolOptions;

use crate::routes::*;

mod data;
mod errors;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    if let Err(err) = dotenv() {
        if err.not_found() && !not_using_dotenv() {
            println!("You have not included a .env file! If this is intentional you can disable this warning with `DISABLE_NO_DOTENV_WARNING=1`")
        } else {
            panic!("Panicked on dotenv error: {}", err);
        }
    };

    let env_config =
        envy::from_env::<EnvConfig>().expect("Failed at getting required enviroment variables");

    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(
            &env::var("DATABASE_URL")
                .expect("Could not find database url from environment variables!"),
        )
        .await
        .expect("Failed to connect to database");

    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Failed to set up migrations");

    let all_endpoints = HelloWorldEndpoint;

    let api_service = OpenApiService::new(all_endpoints, "Loan Broker API", "1.0")
        .server(&env_config.website_url)
        .description("THe API for interacting with the ");

    let data = SharedData {
        pool,
        oauth2_states: Arc::default(),
        env_config: Arc::new(env_config),
        reqwest: reqwest::Client::builder()
            .use_rustls_tls()
            .build()
            .expect("Could not build reqwest client"),
    };

    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .at("/api/login", get(oauth2::login).data(data.clone()))
        .at("/api/callback", get(oauth2::callback).data(data.clone()))
        .at(
            "/api/key/reset",
            get(api_key::reset_api_key).data(data.clone()),
        )
        .nest("/api/swagger", ui)
        .with(ServerSession::new(
            CookieConfig::default(),
            MemoryStorage::new(),
        ));

    poem::Server::new(TcpListener::bind("0.0.0.0:3010"))
        .run(app)
        .await
        .unwrap();
}

fn not_using_dotenv() -> bool {
    match env::var("DISABLE_NO_DOTENV_WARNING")
        .map(|x| x.to_ascii_lowercase())
        .as_deref()
    {
        Ok("1" | "true") => true,
        Ok("0" | "false") => false,
        Ok(_) => {
            panic!("DISABLE_NO_DOTENV_WARNING environment variable is not a valid value")
        }
        Err(VarError::NotPresent) => false,
        Err(VarError::NotUnicode(err)) => panic!(
            "DISABLE_NO_DOTENV_WARNING environment variable is set to valid Unicode, found: {:?}",
            err
        ),
    }
}
