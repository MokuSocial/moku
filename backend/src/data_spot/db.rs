// File: db.rs

mod handler;
mod users;
mod ingredients;
mod recipes;
mod tags;
mod recipe_ingredients;
mod recipe_steps;
mod recipe_tags;


use super::data_types::{Recipe, RecipeIngredient, Step, UnitOfMeasure};

use sqlx::{SqlitePool, query};

//use tokio::task::JoinSet;

pub async fn setup() -> Result<(), String> {
    // Inizializza il database
    let pool = handler::connect().await;
    let handler = handler::initialize(&pool).await;
    print!("Database initialized successfully");
    handler.map_err(|e| format!("Failed to initialize database: {}", e))?;
    println!("Database initialized successfully");

    Ok(())
}

async fn create_recipe(
    db: &SqlitePool,
    recipe : &Recipe,
) -> i64 {
    unimplemented!("Implementa la logica per creare una ricetta");
    /*
    let rec_id = recipes::add_recipe(db, recipe)
    let ingredients_task : JoinSet<_> = recipe.ingredients.iter().map(|ingredient| {
        add_ingredient_to_recipe(db, rec_id, ingredient)
    }).collect();


    let steps_task : JoinSet<_> = recipe.steps.iter().map(|step| {
        add_recipe_step(db, rec_id, step)
    }).collect();

    let tags_task : JoinSet<_> = recipe.tags.iter().map(|tag| {
        add_tag_to_recipe(db, rec_id, &tag.name)
    }).collect();


    // Attendi il completamento di tutti i task
    ingredients_task.join_all().await;
    steps_task.join_all().await;
    tags_task.join_all().await;

    rec_id
    */
}
/*
async fn add_ingredient_to_recipe(
    db: &SqlitePool,
    recipe_id: i64,
    ingredient: &RecipeIngredient
) {
    let unit_str = ingredient.unit.map(|u| u.to_str().to_owned());
    query!(
        "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, quantity, unit) VALUES (?, ?, ?, ?)",
        recipe_id, ingredient.ingredient.id, ingredient.quantity, unit_str
    )
    .execute(db)
    .await
    .expect("Errore nell'aggiunta dell'ingrediente alla ricetta");
}

async fn add_recipe_step(
    db: &SqlitePool,
    recipe_id: i64,
    step: &Step
) {
    query!(
        "INSERT INTO recipe_steps (recipe_id, step_number, description, image_url) VALUES (?, ?, ?, ?)",
        recipe_id, step.step_number, step.description, step.image_url
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

*/
