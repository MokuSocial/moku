use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{self, SaltString, rand_core::OsRng}};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use crate::db::DatabaseHandler;

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
    let password_hash = db.user_password(username).await.ok()?;
    if verify_password(password, password_hash) {
        Some((token(username),refresh_token(username)))
    }
    else {
        None
    }
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).expect("Password hashing failed").to_string()
    //password_hash.to_vec().iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn verify_password(password: &str, hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&hash).expect("Failed to parse hash");
    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => true,
        Err(e) => {
            println!("Password verification error: {}", e);
            false
        },
    }
}
