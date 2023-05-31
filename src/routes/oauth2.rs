use poem::{
    handler,
    web::{Data, Path, Redirect, Query}, IntoResponse, http::StatusCode,
    error::ResponseError
};
use poem_openapi::{payload::{Response, PlainText}, OpenApi};
use rand::Rng;
use serde::Deserialize;
use uuid::Uuid;

use crate::{EnvConfig, SharedData, errors};

#[handler]
pub async fn login(Data(data): Data<&SharedData>) -> Redirect {
    let EnvConfig {
        oauth2_client_id,
        website_url,
        ..
    } = data.env_config.as_ref();

    let state = {
        let mut rng = rand::thread_rng();
        let mut state = rng.gen::<u32>();

        while !data.oauth2_states.insert(state) {
            state = rng.gen();
        }

        state
    };

    let redirect = format!("https://spookvooper.com/oauth/authorize?response_type=code&state={state}&client_id={oauth2_client_id}&redirect_uri={website_url}/api/callback&scope=view,eco");
    Redirect::moved_permanent(redirect)
}

#[handler]
pub async fn callback(
    Data(data): Data<&SharedData>,
    Query(Callback { code, state, entityid }): Query<Callback>
) -> Result<Redirect, CallBackError> {
    if data.oauth2_states.remove(&state).is_none() {
        return Err(CallBackError::InvalidState);
    }

    let EnvConfig {
        oauth2_client_id,
        oauth2_client_secret,
        website_url,
        ..
    } = data.env_config.as_ref();


    dbg!(data);
    dbg!(code);
    dbg!(state);
    dbg!(entityid);

    Ok(Redirect::moved_permanent(&data.env_config.website_url))
}

#[derive(thiserror::Error, Debug)]
pub enum CallBackError {
    #[error("State could not be found in all the current Oauth2 states")]
    InvalidState,
    #[error(transparent)]
    Other(#[from] crate::errors::Error),
}

impl ResponseError for CallBackError {
    fn status(&self) -> StatusCode {
        match self {
            Self::InvalidState => StatusCode::UNAUTHORIZED,
            Self::Other(x) => x.status(),
        }
    }

    fn as_response(&self) -> poem::Response
    {
        match self {
            Self::InvalidState => (self.status(), "Provided state could not be found").into_response(),
            Self::Other(x) => x.as_response(),
        }
    }
}

#[derive(Deserialize)]
pub struct Callback {
    code: Uuid,
    state: u32,
    entityid: i64
}