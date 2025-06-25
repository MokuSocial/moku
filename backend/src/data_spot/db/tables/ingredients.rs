use sqlx::sqlite::SqliteQueryResult;

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
