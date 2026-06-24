use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use validator::Validate;

use crate::errors::api_error::ApiError;

use super::{
    dtos::{CreateUserDto, UserResponseDto},
    models::UserModel,
};

fn collection(db: &Database) -> Collection<UserModel> {
    db.collection("users")
}

pub async fn create_user(db: &Database, input: CreateUserDto) -> Result<UserResponseDto, ApiError> {
    input
        .validate()
        .map_err(|err| ApiError::Validation(err.to_string()))?;

    let password_hash = bcrypt::hash(input.password, bcrypt::DEFAULT_COST)
        .map_err(|_| ApiError::Internal("falha ao criptografar senha".to_string()))?;

    let mut user = UserModel {
        id: None,
        name: input.name,
        role: input.role,
        password_hash: Some(password_hash),
    };

    let insert = collection(db).insert_one(&user).await?;
    user.id = insert.inserted_id.as_object_id();
    Ok(user.into())
}

pub async fn list_users(db: &Database) -> Result<Vec<UserResponseDto>, ApiError> {
    let cursor = collection(db).find(doc! {}).sort(doc! {"name": 1}).await?;
    let users: Vec<UserModel> = cursor.try_collect().await?;
    Ok(users.into_iter().map(Into::into).collect())
}

pub async fn find_user_by_name(db: &Database, name: &str) -> Result<Option<UserModel>, ApiError> {
    Ok(collection(db).find_one(doc! {"name": name}).await?)
}

pub fn verify_user_password(password: &str, password_hash: &str) -> Result<bool, ApiError> {
    bcrypt::verify(password, password_hash)
        .map_err(|_| ApiError::Internal("falha ao validar senha".to_string()))
}

#[cfg(test)]
mod tests {
    use super::verify_user_password;

    #[test]
    fn should_verify_correct_password() {
        let hash = bcrypt::hash("123456", bcrypt::DEFAULT_COST).expect("hash");
        let result = verify_user_password("123456", &hash).expect("verify");
        assert!(result);
    }

    #[test]
    fn should_reject_wrong_password() {
        let hash = bcrypt::hash("123456", bcrypt::DEFAULT_COST).expect("hash");
        let result = verify_user_password("wrong", &hash).expect("verify");
        assert!(!result);
    }
}

