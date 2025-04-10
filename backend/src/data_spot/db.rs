use std::path::Path;
use std::fs::File;

use sqlx::{Pool, Sqlite, Error, SqlitePool, query};
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


async fn create_recipe(
    db: &SqlitePool,
    user_id: i32,
    title: &str,
    introduction: &str,
    conclusion: &str
) -> i64 {
    let rec_id = query!(
        "INSERT INTO recipes (user_id, title, introduction, conclusion) VALUES (?, ?, ?, ?) RETURNING id",
        user_id, title, introduction, conclusion
    )
    .fetch_one(db)
    .await
    .expect("Errore nell'inserimento della ricetta")
    .id;

    rec_id.unwrap_or_else(|| {
        panic!("Errore nell'inserimento della ricetta");
    })
}

async fn add_ingredient_to_recipe(
    db: &SqlitePool,
    recipe_id: i32,
    ingredient_id: i32,
    quantity: f64,
    unit: &str
) {
    query!(
        "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, quantity, unit) VALUES (?, ?, ?, ?)",
        recipe_id, ingredient_id, quantity, unit
    )
    .execute(db)
    .await
    .expect("Errore nell'aggiunta dell'ingrediente alla ricetta");
}

async fn add_recipe_step(
    db: &SqlitePool,
    recipe_id: i32,
    step_number: i32,
    description: &str,
    image_url: Option<&str>
) {
    query!(
        "INSERT INTO recipe_steps (recipe_id, step_number, description, image_url) VALUES (?, ?, ?, ?)",
        recipe_id, step_number, description, image_url
    )
    .execute(db)
    .await
    .expect("Errore nell'aggiunta dello step");
}

async fn add_tag_to_recipe(
    db: &SqlitePool,
    recipe_id: i64,
    tag_name: &str
) -> Result<(), sqlx::Error> {
    // Prova a cercare il tag
    let tag = sqlx::query!(
        "SELECT id FROM tags WHERE name = ?",
        tag_name
    )
    .fetch_optional(db)
    .await?;

    let tag_id = if let Some(record) = tag {
        record.id.unwrap_or_else(|| {
            panic!("Errore nel recupero dell'ID del tag");
        })
    } else {
        sqlx::query!(
            "INSERT INTO tags (name) VALUES (?) RETURNING id",
            tag_name
        )
        .fetch_one(db)
        .await?
        .id
    };

    // Aggiungi relazione ricetta-tag
    sqlx::query!(
        "INSERT INTO recipe_tags (recipe_id, tag_id) VALUES (?, ?)",
        recipe_id, tag_id
    )
    .execute(db)
    .await?;

    Ok(())
}



