use sqlx::sqlite::SqliteQueryResult;

use super::ingredients::IngredientDB;
use super::ingredients::get_ingredient;
use crate::data_spot::data_types::RecipeIngredient;
use crate::data_spot::data_types::UnitOfMeasure;


#[derive(sqlx::FromRow)]
pub struct RecipeIngredientDB {
    recipe_id : i64,
    ingredient_id : i64,
    quantity : f64,
    unit : String,
}

impl RecipeIngredientDB {
    pub fn new(recipe_id: i64, ingredient_id: i64, quantity: f64, unit: String) -> Self {
        Self {
            recipe_id,
            ingredient_id,
            quantity,
            unit,
        }
    }
}

impl From<RecipeIngredient> for IngredientDB {
    fn from(value: RecipeIngredient) -> Self {
        
    }
    
}

impl RecipeIngredient {
    pub async fn from_db(db: &sqlx::SqlitePool, recipe_ingredient_db: RecipeIngredientDB) -> Result<Self, String> {
        let ingredient = get_ingredient(db, recipe_ingredient_db.ingredient_id)
            .await
            .map_err(|e| format!("Failed to get ingredient: {}", e))?
            .try_into()?;
        Ok(Self {
            ingredient,
            quantity: recipe_ingredient_db.quantity,
            unit: UnitOfMeasure::from_str(&recipe_ingredient_db.unit)
        })
    }
}

pub async fn add_recipe_ingredient(
    db: &sqlx::SqlitePool,
    recipe_ingredient: &RecipeIngredientDB
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, quantity, unit) VALUES (?, ?, ?, ?)",
        recipe_ingredient.recipe_id,
        recipe_ingredient.ingredient_id,
        recipe_ingredient.quantity,
        recipe_ingredient.unit
    )
    .execute(db)
    .await
}

pub async fn get_recipe_ingredient(
    db: &sqlx::SqlitePool,
    recipe_id: i64,
    ingredient_id: i64
) -> Result<Option<RecipeIngredientDB>, sqlx::Error> {
    let recipe_ingredient = sqlx::query!(
        "SELECT recipe_id, ingredient_id, quantity, unit FROM recipe_ingredients WHERE recipe_id = ? AND ingredient_id = ?",
        recipe_id,
        ingredient_id
    )
    .fetch_optional(db)
    .await?;

    Ok(recipe_ingredient.map(|record| RecipeIngredientDB {
        recipe_id: record.recipe_id,
        ingredient_id: record.ingredient_id,
        quantity: record.quantity,
        unit: record.unit,
    }))
}
pub async fn delete_recipe_ingredient(
    db: &sqlx::SqlitePool,
    recipe_id: i64,
    ingredient_id: i64
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "DELETE FROM recipe_ingredients WHERE recipe_id = ? AND ingredient_id = ?",
        recipe_id,
        ingredient_id
    )
    .execute(db)
    .await
}
pub async fn get_recipe_ingredients(
    db: &sqlx::SqlitePool,
    recipe_id: i64
) -> Result<Vec<RecipeIngredientDB>, sqlx::Error> {
    let recipe_ingredients = sqlx::query!(
        "SELECT recipe_id, ingredient_id, quantity, unit FROM recipe_ingredients WHERE recipe_id = ?",
        recipe_id
    )
    .fetch_all(db)
    .await?;

    Ok(recipe_ingredients.into_iter().map(|record| RecipeIngredientDB {
        recipe_id: record.recipe_id,
        ingredient_id: record.ingredient_id,
        quantity: record.quantity,
        unit: record.unit,
    }).collect())
}
pub async fn get_ingredients_by_recepie(
    db: &sqlx::SqlitePool,
    recipe_id: i64
) -> Result<Vec<IngredientDB>, sqlx::Error> {
    let ingredients = sqlx::query!(
        "SELECT I.id AS id, I.name AS name, I.identifier AS identifier, I.wikidata AS wikidata, I.cost_per_unit AS cost_per_unit, I.unit AS unit FROM recipe_ingredients AS RI, ingredients AS I WHERE recipe_id = ? AND I.id = RI.ingredient_id",
        recipe_id
    )
    .fetch_all(db)
    .await?;

    Ok(ingredients.into_iter().map(|record| IngredientDB::new(
             record.id,
             record.name,
             record.identifier,
             record.wikidata,
             record.cost_per_unit,
             record.unit,
        )).collect())
}

pub async fn update_recipe_ingredient(
    db: &sqlx::SqlitePool,
    recipe_id: i64,
    ingredient_id: i64,
    new_quantity: Option<f32>,
    new_unit: Option<&str>
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE recipe_ingredients SET quantity = ?, unit = ? WHERE recipe_id = ? AND ingredient_id = ?",
        new_quantity,
        new_unit,
        recipe_id,
        ingredient_id
    )
    .execute(db)
    .await
}


