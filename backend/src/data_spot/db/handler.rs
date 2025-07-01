/*
 * Handler for DB base operations
 */

use std::path::Path;
use std::fs::File;
use sqlx::{query, query_file};
use sqlx::{Pool, Sqlite, Error};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqliteQueryResult;

pub async fn connect() -> Pool<Sqlite> {
    async fn try_connect() -> Result<Pool<Sqlite>, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite://database.db")
            .await?;

        initialize(&pool).await?;
        Ok(pool)
    }
    // Crea una connessione al database SQLite
    let pool_result = try_connect()
        .await;

    match pool_result {
            Ok(pool) => {
                println!("Connected to the database");
                pool
            }
            Err(e) => {
                println!("Failed to connect to the database: {}", e);
                // check if the database file exists and is accessible
                let db_path = Path::new("database.db");
                if db_path.exists() {
                    panic!("Database file exists but could not be opened. Check permissions.");
                } else {
                    println!("Database file does not exist. Creating a new one.");
                    // Create the database File
                    File::create(db_path).expect("Failed to create database file");
                    // Retry the connection
                    try_connect()
                        .await
                        .expect("Failed to connect to the database after creating the file")
                }
            }
        }
}


pub async fn initialize(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    println!("Initializing database...");
    query_file!("res/init.sql")
    .execute(pool)
    .await
}


