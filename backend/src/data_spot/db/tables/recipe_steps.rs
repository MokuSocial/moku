use sqlx::sqlite::SqliteQueryResult;

#[derive(sqlx::FromRow)]
pub struct RecipeStepDB {
    recipe_id : i64,
    step_number : i64,
    description : String,
    image_url : Option<String>,
}

impl RecipeStepDB {
    pub fn new(recipe_id: i64, step_number: i64, description: String, image_url: Option<String>) -> Self {
        Self {
            recipe_id,
            step_number,
            description,
            image_url,
        }
    }
}

pub async fn add_recipe_step(
    db: &sqlx::SqlitePool,
    recipe_step: &RecipeStepDB
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO recipe_steps (recipe_id, step_number, description, image_url) VALUES (?, ?, ?, ?)",
        recipe_step.recipe_id,
        recipe_step.step_number,
        recipe_step.description,
        recipe_step.image_url
    )
    .execute(db)
    .await
}

pub async fn get_recipe_step(
    db: &sqlx::SqlitePool,
    recipe_id: i64,
    step_number: i64
) -> Result<RecipeStepDB, sqlx::Error> {
    let recipe_step = sqlx::query!(
        "SELECT recipe_id, step_number, description, image_url FROM recipe_steps WHERE recipe_id = ? AND step_number = ?",
        recipe_id,
        step_number
    )
    .fetch_optional(db)
    .await?;

    match recipe_step.map(|record| RecipeStepDB {
        recipe_id: record.recipe_id,
        step_number: record.step_number,
        description: record.description,
        image_url: record.image_url,
    }) {
        Some(step) => Ok(step),
        None => Err(sqlx::Error::RowNotFound),
    }
}
pub async fn delete_recipe_step(
    db: &sqlx::SqlitePool,
    recipe_id: i64,
    step_number: i64
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "DELETE FROM recipe_steps WHERE recipe_id = ? AND step_number = ?",
        recipe_id,
        step_number
    )
    .execute(db)
    .await
}

pub async fn get_recipe_steps(
    db: &sqlx::SqlitePool,
    recipe_id: i64
) -> Result<Vec<RecipeStepDB>, sqlx::Error> {
    let recipe_steps = sqlx::query!(
        "SELECT recipe_id, step_number, description, image_url FROM recipe_steps WHERE recipe_id = ?",
        recipe_id
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|record| RecipeStepDB {
        recipe_id: record.recipe_id,
        step_number: record.step_number,
        description: record.description,
        image_url: record.image_url,
    }).collect();

    Ok(recipe_steps)


}

pub async fn update_recipe_step(
    db: &sqlx::SqlitePool,
    recipe_id: i64,
    step_number: i64,
    new_description: Option<&str>,
    new_image_url: Option<&str>
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE recipe_steps SET description = ?, image_url = ? WHERE recipe_id = ? AND step_number = ?",
        new_description,
        new_image_url,
        recipe_id,
        step_number
    )
    .execute(db)
    .await
}

