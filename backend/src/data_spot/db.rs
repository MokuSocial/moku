// File: db.rs

mod handler;
//mod users;
//mod ingredients;
mod tables;
//mod tags;
//mod recipe_ingredients;
//mod recipe_steps;
//mod recipe_tags;
//

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
    let rec_id = tables::recipes::add_recipe(db, &tables::recipes::RecipeDB::new(
        recipe.id,
        recipe.user_id.clone(),
        recipe.title.clone(),
        recipe.introduction.clone(),
        recipe.conclusion.clone(),
        recipe.created_at.timestamp(),
    )).await?;

    for ingredient in &recipe.ingredients {
        tables::recipe_ingredients::add_recipe_ingredient(db, &tables::recipe_ingredients::RecipeIngredientDB::new(
            rec_id,
            ingredient.ingredient.id,
            ingredient.quantity,
            ingredient.ingredient.unit.map_or("piece".to_owned(), |u| u.to_string())
        )).await?;
    }

    for step in &recipe.steps {
        tables::recipe_steps::add_recipe_step(db, &tables::recipe_steps::RecipeStepDB::new(
            rec_id,
            step.step_number as i64,
            step.description.clone(),
            step.image_url.clone(),
        )).await?;
    }

    for tag in &recipe.tags {
        // TODO
    }

    Ok(rec_id)
}
