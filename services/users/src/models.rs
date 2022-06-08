use crate::schema::users; 
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name:Option<String>, 
    pub last_name: Option<String>, 
    pub username: String,
    pub email: String, 
    #[serde(skip_serializing)]
    pub passwd: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct InsertableUser {
    pub first_name: String, 
    pub last_name: String, 
    pub username: String,
    pub email: String, 
    pub passwd: String,
    pub created_at: chrono::NaiveDateTime,
}

impl InsertableUser {
    pub fn new(first_name: String, last_name: String, username: String, email: String, passwd: String) -> Self {
        Self { first_name, last_name, username, email, passwd, created_at: chrono::Local::now().naive_local() }
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i32,
    pub username: String,
    pub email: String
}
