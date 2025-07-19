use sqlx::sqlite::SqliteQueryResult;
use crate::data_spot::data_types::Ingredient;
use crate::data_spot::data_types::UnitOfMeasure;
use std::convert::TryFrom;

#[derive(sqlx::FromRow)]
pub struct IngredientDB {
    id : i64,
    name : String,
    identifier : String,
    wikidata : Option<String>,
    cost_per_unit : Option<f64>,
    unit : Option<String>,
}

impl IngredientDB {
    pub fn new(id: i64, name: String, identifier: String, wikidata: Option<String>, cost_per_unit: Option<f64>, unit: Option<String>) -> Self {
        Self {
            id,
            name,
            identifier,
            wikidata,
            cost_per_unit,
            unit,
        }
    }
}

impl From<&Ingredient> for IngredientDB {
    fn from(value: &Ingredient) -> Self {
        value.to_owned().into()
    }
}

impl From<Ingredient> for IngredientDB {
    fn from(ingredient: Ingredient) -> Self {
        Self {
            id: ingredient.id,
            name: ingredient.name,
            identifier: ingredient.identifier,
            wikidata: ingredient.wikidata,
            cost_per_unit: ingredient.cost_per_unit,
            unit: ingredient.unit.map(|u| u.to_string()),
        }
    }
}

impl TryFrom<IngredientDB> for Ingredient {
    type Error = String;
    fn try_from(ingredient_db: IngredientDB) -> Result<Self, Self::Error> {
        Ok(Self {
                    id: ingredient_db.id,
                    name: ingredient_db.name,
                    identifier: ingredient_db.identifier,
                    wikidata: ingredient_db.wikidata,
                    cost_per_unit: ingredient_db.cost_per_unit,
                    unit: ingredient_db.unit.and_then(|u| UnitOfMeasure::from_str(&u)),
                })
    }
    
}

pub async fn add_ingredient(
    db: &sqlx::SqlitePool,
    ingredient: &IngredientDB
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO ingredients (name, identifier, wikidata, cost_per_unit, unit) VALUES (?, ?, ?, ?, ?)",
        ingredient.name,
        ingredient.identifier,
        ingredient.wikidata,
        ingredient.cost_per_unit,
        ingredient.unit
    )
    .execute(db)
    .await
}

pub async fn get_ingredient(
    db: &sqlx::SqlitePool,
    id: i64
) -> Result<IngredientDB, sqlx::Error> {
    let ingredient = sqlx::query!(
        "SELECT id, name, identifier, wikidata, cost_per_unit, unit FROM ingredients WHERE id = ?",
        id
    )
    .fetch_optional(db)
    .await?;

    match ingredient.map(|record| IngredientDB {
        id: record.id,
        name: record.name,
        identifier: record.identifier,
        wikidata: record.wikidata,
        cost_per_unit: record.cost_per_unit,
        unit: record.unit,
    }) {
        Some(ingredient) => Ok(ingredient),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn delete_ingredient(
    db: &sqlx::SqlitePool,
    id: i64
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "DELETE FROM ingredients WHERE id = ?",
        id
    )
    .execute(db)
    .await
}

pub async fn update_ingredient(
    db: &sqlx::SqlitePool,
    id: i64,
    new_name: Option<&str>,
    new_identifier: Option<&str>,
    new_wikidata: Option<&str>,
    new_cost_per_unit: Option<f64>,
    new_unit: Option<String>
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE ingredients SET name = ?, identifier = ?, wikidata = ?, cost_per_unit = ?, unit = ? WHERE id = ?",
        new_name,
        new_identifier,
        new_wikidata,
        new_cost_per_unit,
        new_unit,
        id
    )
    .execute(db)
    .await
}
