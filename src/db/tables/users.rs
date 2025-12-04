use sqlx::sqlite::SqliteQueryResult;


#[derive(sqlx::FromRow, Debug, Clone)]
pub struct UserDB {
    pub username : String,
    pub password_hash : String,
    pub email : String,
    pub created_at : i64,
}

//impl From<UserDB> for User {
//    
//}

impl UserDB {

    pub async fn get_password_hash(
        db: &sqlx::SqlitePool,
        username: &str
    ) -> Result<String, sqlx::Error> {
        let record = sqlx::query!(
            "SELECT password_hash FROM users WHERE username = ?",
            username
        )
        .fetch_one(db)
        .await?;

        Ok(record.password_hash)
    }
}
/*
pub async fn add_user(
    db: &sqlx::SqlitePool,
    user: &UserDB
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (username, password_hash, email) VALUES (?, ?, ?)",
        user.username,
        user.password_hash,
        user.email
    )
    .execute(db)
    .await
}

pub async fn get_user(
    db: &sqlx::SqlitePool,
    username: &str
) -> Result<Option<UserDB>, sqlx::Error> {
    let user = sqlx::query!(
        "SELECT username, password_hash, email, created_at FROM users WHERE username = ?",
        username
    )
    .fetch_optional(db)
    .await?;

    Ok(user.map(|record| UserDB {
        username: record.username,
        password_hash: record.password_hash,
        email: record.email,
        created_at: record.created_at,
    }))
}

pub async fn delete_user(
    db: &sqlx::SqlitePool,
    username: &str
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "DELETE FROM users WHERE username = ?",
        username
    )
    .execute(db)
    .await
}

pub async fn update_user(
    db: &sqlx::SqlitePool,
    username: &str,
    new_email: Option<&str>,
    new_password_hash: Option<&str>
) -> Result<SqliteQueryResult, sqlx::Error> {
    let mut query = String::from("UPDATE users SET ");
    let mut params = Vec::new();

    if let Some(email) = new_email {
        query.push_str("email = ?, ");
        params.push(email);
    }

    if let Some(password_hash) = new_password_hash {
        query.push_str("password_hash = ?, ");
        params.push(password_hash);
    }

    // Rimuovi l'ultima virgola e spazio
    query.pop();
    query.pop();

    query.push_str(" WHERE username = ?");
    params.push(username);


    let p : Vec<u8> = params.iter().flat_map(|e| e.as_bytes().to_owned()).collect();

    sqlx::query(&query)
        .bind(p)
        .execute(db)
        .await

}
*/
