// File: db.rs

mod handler;
mod tables;

use crate::data_types::{Recipe, Ingredient};

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

pub trait FromDB<T> {
    fn from_db(db: &SqlitePool, item: T) -> Result<Self, String>
    where
        Self: Sized;
    
}


async fn create_recipe(
    db: &SqlitePool,
    recipe : &Recipe,
) -> Result<i64, sqlx::Error> {
    let rec_id = tables::recipes::add_recipe(db, &recipe.into()).await?;

    for ingredient in &recipe.ingredients {
        tables::recipe_ingredients::add_recipe_ingredient(db, &ingredient.into()).await?;
    }

    for step in &recipe.steps {
        tables::recipe_steps::add_recipe_step(db, &step.into()).await?;
    }

    for tag in &recipe.tags {
        tables::recipe_tags::add_recipe_tag(db, &tag.into()).await?;
    }

    Ok(rec_id)
}

async fn get_recipe(
    db: &SqlitePool,
    id: i64
) -> Result<Recipe, sqlx::Error> {
    let rec = tables::recipes::get_recipe(db, id).await?;

    tables::recipes::RecipeDB::new(id, user_id, title, introduction, conclusion, created_at)
}


async fn create_ingredient(
    db: &SqlitePool,
    ingredient: &Ingredient
) -> Result<i64, sqlx::Error> {
    tables::ingredients::add_ingredient(db, ingredient).await
}
