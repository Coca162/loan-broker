use poem::{
    handler,
    http::StatusCode,
    session::Session,
    web::{Data, Query, Redirect},
};
use rand::Rng;
use reqwest::Method;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::{Error, Result},
    models::RequestTokenResponse,
    EnvConfig, SharedData,
};

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
        data.oauth2_states.len();
        state
    };

    let redirect = format!("https://spookvooper.com/oauth/authorize?response_type=code&state={state}&client_id={oauth2_client_id}&redirect_uri={website_url}/api/callback&scope=view,eco");
    Redirect::see_other(redirect)
}

#[handler]
pub async fn callback(
    Data(data): Data<&SharedData>,
    Query(Callback {
        code,
        state,
        entityid,
    }): Query<Callback>,
    session: &Session,
) -> Result<Redirect> {
    if data.oauth2_states.remove(&state).is_none() {
        return Err(Error::Custom(
            StatusCode::UNAUTHORIZED,
            "Provided state could not be found",
        ));
    }

    let EnvConfig {
        oauth2_client_id,
        oauth2_client_secret,
        website_url,
        ..
    } = data.env_config.as_ref();

    let response: RequestTokenResponse = data
        .reqwest
        .request(Method::GET, "https://spookvooper.com/OAuth/RequestToken")
        .query(&[("grant_type", "authorization_code")])
        .query(&[("redirect_uri", data.env_config.oauth2_redirect_uri())])
        .query(&[("code", code)])
        .query(&[("client_id", oauth2_client_id)])
        .query(&[("client_secret", oauth2_client_secret)])
        .send()
        .await?
        .json()
        .await?;

    assert_eq!(response.entityid, entityid);

    let mut trans = data.pool.begin().await.map_err(Error::Database)?;

    sqlx::query!(
        "INSERT INTO account VALUES ($1) ON CONFLICT (id) DO NOTHING",
        response.userid
    )
    .execute(&mut trans)
    .await?;

    sqlx::query!("INSERT INTO entity VALUES ($1,$2,$3,$4) ON CONFLICT (id,holder_id) DO UPDATE SET access_token = EXCLUDED.access_token",
        response.entityid,
        response.userid,
        response.entity_type as i16,
        response.access_token
    )
    .execute(&mut trans)
    .await?;

    trans.commit().await?;

    session.set("svid", response.userid);

    Ok(Redirect::see_other(website_url))
}

#[derive(Deserialize)]
pub struct Callback {
    code: Uuid,
    state: u32,
    entityid: i64,
}
