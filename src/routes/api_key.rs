use poem::{
    handler,
    http::StatusCode,
    session::Session,
    web::Data,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    errors::{Error, Result},
    SharedData,
};

#[handler]
pub async fn reset_api_key(Data(data): Data<&SharedData>, session: &Session) -> Result<String> {
    let id: Option<i64> = session.get("svid");

    let id = if let Some(id) = id {
        id
    } else {
        return Err(Error::Custom(
            StatusCode::FORBIDDEN,
            "You need to be logged in to generate the api key!",
        ));
    };

    let key = Uuid::new_v4();

    let mut hasher = Sha256::new();

    hasher.update(key);

    let result: &[u8] = &hasher.finalize();

    sqlx::query!(
        "UPDATE account SET api_key_hash = $1 WHERE id = $2",
        result,
        id
    )
    .execute(&data.pool)
    .await?;

    Ok(format!(
        "THIS KEY WILL BE RESET UPON THE NEXT VISIT OF THIS SITE:\n{key}"
    ))
}
