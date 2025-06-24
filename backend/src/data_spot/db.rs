// File: db.rs

mod handler;
mod users;
mod ingredients;
mod recipes;
mod tags;
mod recipe_ingredients;
mod recipe_steps;
mod recipe_tags;


use super::data_types::Recipe;

use sqlx::{Pool, Sqlite, SqlitePool};

//use tokio::task::JoinSet;

pub async fn setup() -> Result<Pool<Sqlite>, String> {
    // Inizializza il database
    let pool = handler::connect().await;
    let handler = handler::initialize(&pool).await;
    handler.map_err(|e| format!("Failed to initialize database: {}", e))?;
    println!("Database initialized successfully");
    Ok(pool)
}

async fn create_recipe(
    db: &SqlitePool,
    recipe : &Recipe,
) -> Result<i64, sqlx::Error> {
    let rec_id = recipes::add_recipe(db, &recipes::RecipeDB::new(
        recipe.id,
        recipe.user_id.clone(),
        recipe.title.clone(),
        recipe.introduction.clone(),
        recipe.conclusion.clone(),
        recipe.created_at.timestamp(),
    )).await?;

    for ingredient in &recipe.ingredients {
        // TODO
    }

    for step in &recipe.steps {
        // TODO
    }

    for tag in &recipe.tags {
        // TODO
    }

    Ok(rec_id)
}
