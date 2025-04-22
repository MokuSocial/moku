use sqlx::sqlite::SqliteQueryResult;


struct RecipeTagDB {
    recipe_id : i64,
    tag_id : i64,
}
impl RecipeTagDB {
    pub fn new(recipe_id: i64, tag_id: i64) -> Self {
        Self { recipe_id, tag_id }
    }
}
pub async fn add_recipe_tag(
    db: &sqlx::SqlitePool,
    recipe_tag: &RecipeTagDB
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO recipe_tags (recipe_id, tag_id) VALUES (?, ?)",
        recipe_tag.recipe_id,
        recipe_tag.tag_id
    )
    .execute(db)
    .await
}

pub async fn get_recipe_tag(
    db: &sqlx::SqlitePool,
    recipe_id: i64,
    tag_id: i64
) -> Result<Option<RecipeTagDB>, sqlx::Error> {
    let recipe_tag = sqlx::query!(
        "SELECT recipe_id, tag_id FROM recipe_tags WHERE recipe_id = ? AND tag_id = ?",
        recipe_id,
        tag_id
    )
    .fetch_optional(db)
    .await?;

    Ok(recipe_tag.map(|record| RecipeTagDB {
        recipe_id: record.recipe_id,
        tag_id: record.tag_id,
    }))
}
pub async fn delete_recipe_tag(
    db: &sqlx::SqlitePool,
    recipe_id: i64,
    tag_id: i64
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "DELETE FROM recipe_tags WHERE recipe_id = ? AND tag_id = ?",
        recipe_id,
        tag_id
    )
    .execute(db)
    .await
}

pub async fn get_recipe_tags(
    db: &sqlx::SqlitePool,
    recipe_id: i64
) -> Result<Vec<RecipeTagDB>, sqlx::Error> {
    let recipe_tags = sqlx::query!(
        "SELECT recipe_id, tag_id FROM recipe_tags WHERE recipe_id = ?",
        recipe_id
    )
    .fetch_all(db)
    .await?;

    Ok(recipe_tags.into_iter().map(|record| RecipeTagDB {
        recipe_id: record.recipe_id,
        tag_id: record.tag_id,
    }).collect())
}
pub async fn get_recipes_by_tag(
    db: &sqlx::SqlitePool,
    tag_id: i64
) -> Result<Vec<RecipeTagDB>, sqlx::Error> {
    let recipe_tags = sqlx::query!(
        "SELECT recipe_id, tag_id FROM recipe_tags WHERE tag_id = ?",
        tag_id
    )
    .fetch_all(db)
    .await?;

    Ok(recipe_tags.into_iter().map(|record| RecipeTagDB {
        recipe_id: record.recipe_id,
        tag_id: record.tag_id,
    }).collect())
}

