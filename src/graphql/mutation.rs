use async_graphql::{Object, SimpleObject};


pub struct Mutation;

#[Object]
impl Mutation {
    async fn login(&self, username: String, password: String) -> LoginResponse {
        // Here you would normally check the username and password against your database
        // For simplicity, we'll just return true if the username is "admin" and the password is "password"
        if username == "admin" && password == "password" {
            LoginResponse {
                success: true,
                token: "fake-jwt-token".to_string().into(),
                refresh_token: "fake-refresh-token".to_string().into(),
                username: username.into(),
                expires_in: 3600.into(),
            }
        } else {
            LoginResponse {
                success: false,
                token: None,
                refresh_token: None,
                username: None,
                expires_in: None,
            }
        }
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
