use async_graphql::{Object, SimpleObject, Context};
use crate::db::DatabaseHandler;

use crate::auth::{self, authenticate};


pub struct Mutation;

#[Object]
impl Mutation {
    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> LoginResponse {
        // Here you would normally check the username and password against your database
        // For simplicity, we'll just return true if the username is "admin" and the password is "password"
        let db = ctx.data::<DatabaseHandler>().unwrap();
        match authenticate(username.as_str(), password.as_str(), db).await {
            Some((token,refresh_token)) => LoginResponse {
                success: true,
                token: token.into(),
                refresh_token: refresh_token.into(),
                username: username.into(),
                expires_in: 3600.into(),
            },
            None => LoginResponse {
                success: false,
                token: None,
                refresh_token: None,
                username: None,
                expires_in: None,
            }
        }
    }

    async fn hasher(&self, password: String) -> String {
        auth::hash_password(password.as_str())
    }
}

#[derive(SimpleObject, Clone)]
pub struct LoginResponse {
    pub success: bool,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub username: Option<String>,
    pub expires_in: Option<i32>,
}
