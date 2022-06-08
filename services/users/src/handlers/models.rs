use crate::{errors::ServiceError, models::InsertableUser};
use common::util::hash_password_salted;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignUp {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub passwd: String,
}

impl InsertableUser {
    pub fn from_signup(data: SignUp) -> Result<Self, ServiceError> {
        let user = InsertableUser::new(
            data.first_name,
            data.last_name,
            data.username,
            data.email,
            hash_password_salted(&data.passwd)?,
        );

        Ok(user)
    }
}
#[derive(Debug, Deserialize)]
pub struct Filter {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>
}