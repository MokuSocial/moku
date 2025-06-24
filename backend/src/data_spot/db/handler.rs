/*
 * Handler for DB base operations
 */

use std::path::Path;
use std::fs::File;
use sqlx::query;
use sqlx::{Pool, Sqlite, Error};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqliteQueryResult;

pub async fn connect() -> Pool<Sqlite> {
    async fn try_connect() -> Result<Pool<Sqlite>, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite://database.db")
            .await;

        pool.map(|pool| {
            initialize(&pool);
            pool
        })
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
    query!(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY NOT NULL,  -- Identificatore dell'utente (stringa)
            password_hash TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS ingredients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            identifier TEXT UNIQUE NOT NULL, 
            wikidata TEXT,
            cost_per_unit REAL,
            unit TEXT CHECK (unit IN ('kg', 'g', 'l', 'ml', 'piece'))  -- Unità di misura
        );

        CREATE TABLE IF NOT EXISTS recipes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,  -- Identificativo utente come stringa
            title TEXT NOT NULL,
            introduction TEXT NOT NULL,
            conclusion TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            last_updated INTEGER DEFAULT NULL,
            FOREIGN KEY(user_id) REFERENCES users(username) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS recipe_ingredients (
            recipe_id INTEGER NOT NULL,
            ingredient_id INTEGER NOT NULL,
            quantity REAL NOT NULL,
            unit TEXT CHECK (unit IN ('kg', 'g', 'l', 'ml', 'piece')) NOT NULL,
            FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
            FOREIGN KEY(ingredient_id) REFERENCES ingredients(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS recipe_steps (
            recipe_id INTEGER NOT NULL,
            step_number INTEGER NOT NULL,
            description TEXT NOT NULL,
            image_url TEXT,
            PRIMARY KEY(recipe_id, step_number),
            FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS recipe_tags (
            recipe_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
            FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE
        );
        "#
    )
    .execute(pool)
    .await
}


