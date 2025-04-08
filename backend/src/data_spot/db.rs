use std::path::Path;
use std::fs::File;

use sqlx::{Pool, Sqlite, Error};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqliteQueryResult;

pub async fn connect_db() -> Pool<Sqlite> {
    async fn try_connect() -> Result<Pool<Sqlite>, sqlx::Error> {
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite://database.db")
            .await
    }
    // Crea una connessione al database SQLite
    let pool = try_connect()
        .await
        .unwrap_or_else( |_| {
            // check if the database file exists and is accessible
            let db_path = Path::new("database.db");
            if db_path.exists() {
                panic!("Database file exists but could not be opened. Check permissions.");
            } else {
                println!("Database file does not exist. Creating a new one.");
                // Create the database File
                File::create(db_path).expect("Failed to create database file");
                panic!("Database file created restarting the connection");
            }
        });
        //.expect("Failed to connect to the database\n Check if the database file exists and is accessible");

    pool

}


pub async fn initialize_db(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY NOT NULL,  -- Identificatore dell'utente (stringa)
            password_hash TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS ingredients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            identifier TEXT UNIQUE NOT NULL, 
            wikidata TEXT,
            cost_per_unit REAL,
            unit TEXT CHECK (unit IN ('kg', 'g', 'l', 'ml', 'unit'))
        );

        CREATE TABLE IF NOT EXISTS recipes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,  -- Identificativo utente come stringa
            title TEXT NOT NULL,
            introduction TEXT NOT NULL,
            conclusion TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(user_id) REFERENCES users(username) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS recipe_ingredients (
            recipe_id INTEGER NOT NULL,
            ingredient_id INTEGER NOT NULL,
            quantity REAL NOT NULL,
            unit TEXT NOT NULL,
            FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
            FOREIGN KEY(ingredient_id) REFERENCES ingredients(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS recipe_steps (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id INTEGER NOT NULL,
            step_number INTEGER NOT NULL,
            description TEXT NOT NULL,
            image_url TEXT,
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

