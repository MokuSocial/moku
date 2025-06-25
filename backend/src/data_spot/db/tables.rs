/* DB tables */
pub mod recipes;
// pub mod users;
pub mod ingredients;
//pub mod tags;
pub mod recipe_ingredients;
pub mod recipe_steps;
pub mod recipe_tags;
// pub mod recipe_images;
// pub mod recipe_comments;
// pub mod recipe_ratings;
// pub mod recipe_favorites;
// pub mod recipe_history;
// pub mod recipe_notifications;
// pub mod recipe_sharing;
// pub mod recipe_collaborators;
// pub mod recipe_versions;
// pub mod recipe_statistics;

/* * Raw SQL query execution
 * This function executes a raw SQL query and returns the result as a string.
 */
/*
use sqlx::sqlite::SqliteQueryResult;

pub async fn raw_query(query: &str) -> String {
    sqlx::query(query)
        .fetch_one(&db::get_db_pool())
        .await
        .map(|row| format!("{:?}", row))
        .unwrap_or_else(|_| "Query failed".to_string())
}
*/
