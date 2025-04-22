use sqlx::sqlite::SqliteQueryResult;


#[derive(sqlx::FromRow)]
struct RecipeIngredientDB {
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


