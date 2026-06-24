use mongodb::{Client, Database};

use crate::errors::api_error::ApiError;

use super::AppConfig;

pub async fn init_database(config: &AppConfig) -> Result<Database, ApiError> {
    let client = Client::with_uri_str(&config.mongodb_uri)
        .await
        .map_err(ApiError::Database)?;
    Ok(client.database(&config.database_name))
}

