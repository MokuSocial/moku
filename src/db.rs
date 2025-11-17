// File: db.rs

mod handler;
mod tables;

use crate::data_types::{Author, Ingredient, Recipe, Step, RecipeIngredient};

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

pub async fn get_recipe(db: &SqlitePool, id: i64) -> Result<Recipe, String> {
    let rec_db = tables::recipes::RecipeDB::get(db, id).await.map_err(|e| e.to_string())?;
    Ok(Recipe::from(rec_db))
}

pub async fn get_recipe_with_indications(db: &SqlitePool, id: i64) -> Result<Recipe, String> {
    let rec_db = tables::recipes::RecipeDB::get_recipe_with_indications(db, id).await.map_err(|e| e.to_string())?;
    Ok(Recipe::from(rec_db))
}

pub async fn get_steps(db: &SqlitePool, recipe_id: i64) -> Result<Vec<Step>, String> {
    let steps_db = tables::recipe_steps::RecipeStepDB::gets(db, recipe_id).await.map_err(|e| e.to_string())?;
    let steps: Vec<Step> = steps_db.into_iter().map(|s| Step::from(s)).collect();
    Ok(steps)
}

pub async fn get_recipe_ingredients(db: &SqlitePool, recipe_id: i64) -> Result<Vec<crate::data_types::RecipeIngredient>, String> {
    let ri_db = tables::recipe_ingredients::RecipeIngredientDB::gets_by_recepie_id(db, recipe_id).await.map_err(|e| e.to_string())?;
    let ingredients: Vec<RecipeIngredient> = ri_db.into_iter().map(|ri| crate::data_types::RecipeIngredient::from(ri)).collect();
    Ok(ingredients)
}

/*pub async fn get_ingredient(db: &SqlitePool, id: i64) -> Result<Ingredient, String> {
    let ing_db = tables::ingredients::IngredientDB::get(db, id).await.map_err(|e| e.to_string())?;

    Ok(Ingredient::from(ing_db))
}*/
