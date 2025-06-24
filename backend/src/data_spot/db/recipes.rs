use sqlx::sqlite::SqliteQueryResult;

#[derive(sqlx::FromRow)]
pub struct RecipeDB {
    id : i64,
    user_id : String,
    title : String,
    introduction : String,
    conclusion : String,
    created_at : i64,
    last_updated : Option<i64>,
}

impl RecipeDB {
    pub fn new(id: i64, user_id: String, title: String, introduction: String, conclusion: String, created_at: i64) -> Self {
        Self {
            id,
            user_id,
            title,
            introduction,
            conclusion,
            created_at,
            last_updated: None,
        }
    }
}

pub async fn add_recipe(
    db: &sqlx::SqlitePool,
    recipe: &RecipeDB
) -> Result<i64, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO recipes (user_id, title, introduction, conclusion) VALUES (?, ?, ?, ?) RETURNING id",
        recipe.user_id,
        recipe.title,
        recipe.introduction,
        recipe.conclusion
    )
    .fetch_one(db)
    .await?
    .id
    .ok_or(sqlx::Error::RowNotFound)
}

pub async fn get_recipe(
    db: &sqlx::SqlitePool,
    id: i64
) -> Result<Option<RecipeDB>, sqlx::Error> {
    let recipe = sqlx::query!(
        "SELECT id, user_id, title, introduction, conclusion, created_at, last_updated FROM recipes WHERE id = ?",
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(recipe.map(|record| RecipeDB {
        id: record.id,
        user_id: record.user_id,
        title: record.title,
        introduction: record.introduction,
        conclusion: record.conclusion,
        created_at: record.created_at,
        last_updated: record.last_updated,
    }))
}
pub async fn delete_recipe(
    db: &sqlx::SqlitePool,
    id: i64
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "DELETE FROM recipes WHERE id = ?",
        id
    )
    .execute(db)
    .await
    
}

pub async fn get_recipes_by_user(
    db: &sqlx::SqlitePool,
    user_id: &str
) -> Result<Vec<RecipeDB>, sqlx::Error> {
    let recipes = sqlx::query!(
        "SELECT id, user_id, title, introduction, conclusion, created_at, last_updated FROM recipes WHERE user_id = ?",
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(recipes.into_iter().map(|record| RecipeDB {
        id: record.id,
        user_id: record.user_id,
        title: record.title,
        introduction: record.introduction,
        conclusion: record.conclusion,
        created_at: record.created_at,
        last_updated: record.last_updated,
    }).collect())
}
pub async fn update_recipe(
    db: &sqlx::SqlitePool,
    id: i64,
    new_title: Option<&str>,
    new_introduction: Option<&str>,
    new_conclusion: Option<&str>
) -> Result<SqliteQueryResult, sqlx::Error> {
    let new_title = new_title.map(|s| s.to_string());
    let new_introduction = new_introduction.map(|s| s.to_string());
    let new_conclusion = new_conclusion.map(|s| s.to_string());
    sqlx::query!(
        "UPDATE recipes SET title = ?, introduction = ?, conclusion = ? WHERE id = ?",
        new_title,
        new_introduction,
        new_conclusion,
        id
    )
    .execute(db)
    .await
}

