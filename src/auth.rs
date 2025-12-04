use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use crate::db::{self, DatabaseHandler};

const SECRET : &str = "secret_key";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    //aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
    scope: Option<String>, // Optional. Scope of the token
}

fn create_jwt(username: &str, ttl: usize, scope: Option<&str>) -> String {
    let claims = Claims {
        //aud: "your_audience".to_string(),
        exp: chrono::Utc::now().timestamp() as usize + ttl,
        iat: chrono::Utc::now().timestamp() as usize,
        iss: "moku dev".to_string(),
        nbf: chrono::Utc::now().timestamp() as usize,
        sub: username.to_string(),
        scope: scope.map(|s| s.to_string()),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_ref())).unwrap()
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.leeway=0;
    validation.validate_exp=true;
    validation.algorithms=vec![Algorithm::HS256];

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET.as_ref()), &validation)?;
    Ok(token_data.claims)
}

fn token(username: &str) -> String {
    create_jwt(username, 3600, Some("token"))
}

fn refresh_token(username: &str) -> String {
    create_jwt(username, 3600*24, Some("refresh_token"))
}

pub async fn authenticate(username: &str, password: &str, db: &DatabaseHandler) -> Option<(String,String)> {
    let hash = password;
    match db.authenticate_user(username, hash).await {
        Ok(true) => Some((token(username),refresh_token(username))),
        _ => None,
    } 
}
