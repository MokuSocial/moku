//use anyhow::Ok;

use crate::data_types::{Author, Indication, Recipe};


#[derive(sqlx::FromRow)]
pub struct RecipeDB {
    pub id : i64,
    pub author : String,
    pub title : String,
    //introduction : String,
    //conclusion : String,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub rest_time: Option<i64>,
    pub difficulty: Option<String>,
    pub banner_image_url : Option<String>,
    pub servings : i64,
    pub vote_count : i64,
    pub vote_average : f64,
    //pub created_at : i64,
    //pub last_updated : Option<i64>,
}

impl From<&Recipe> for RecipeDB{
    fn from(value: &Recipe) -> Self {
        value.to_owned().into()
    }
    
}

impl From<Recipe> for RecipeDB {
    fn from(recipe: Recipe) -> Self {
        Self {
            id: recipe.id,
            author: recipe.author.username,
            title: recipe.title,
            banner_image_url: recipe.banner_url,
            prep_time: Some(recipe.indications.prep_time as i64),
            cook_time: Some(recipe.indications.cook_time as i64),
            rest_time: recipe.indications.rest_time.map(|e| e as i64),
            difficulty: Some(recipe.indications.difficulty),
            vote_count: recipe.votes as i64,
            vote_average: recipe.vote_average,
            servings: recipe.servings as i64,
        }
    }
}

impl From<RecipeDB> for Recipe {
    fn from(rec_db: RecipeDB) -> Self {

    let author = Author {
        username: rec_db.author.clone(),
    };

    Recipe {
        id: rec_db.id,
        author,
        title: rec_db.title,
        banner_url: rec_db.banner_image_url,
        indications: Indication {
            prep_time: rec_db.prep_time.unwrap_or(0) as u32,
            cook_time: rec_db.cook_time.unwrap_or(0) as u32,
            rest_time: rec_db.rest_time.map(|t| t as u32),
            difficulty: rec_db.difficulty.unwrap_or_default(),
        },
        votes: rec_db.vote_count as u32,
        vote_average: rec_db.vote_average,
        servings: rec_db.servings as u16,
        }
    }
}

impl RecipeDB {
    async fn fetch_internal(
        db: &sqlx::SqlitePool,
        id: i64,
        with_indications: bool,
    ) -> Result<RecipeDB, sqlx::Error> {
        // Query diversa in base alla modalità
        let record = 
            sqlx::query_as!(RecipeDB,
                r#"
                SELECT id, author, title, banner_image_url, servings,
                       prep_time, cook_time, rest_time, difficulty,
                       vote_count, vote_average
                FROM recipes WHERE id = ?
                "#,
                id
            )
            .fetch_optional(db)
            .await?
            .ok_or(sqlx::Error::RowNotFound);

        record
    }

    pub async fn count(db: &sqlx::SqlitePool) -> Result<i64, sqlx::Error> {
        /*let record = */sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as count FROM recipes
            "#
        )
        .fetch_one(db)
        .await
    }

    pub async fn get(db: &sqlx::SqlitePool, id: i64) -> Result<RecipeDB, sqlx::Error> {
        Self::fetch_internal(db, id, false).await
    }

    pub async fn get_recipe_with_indications(
        db: &sqlx::SqlitePool,
        id: i64,
    ) -> Result<RecipeDB, sqlx::Error> {
        Self::fetch_internal(db, id, true).await
    }

    pub async fn gets(
        db: &sqlx::SqlitePool,
        first: Option<i64>,
        after: Option<i64>,
    ) -> Result<Vec<RecipeDB>, sqlx::Error> {

        let first = first.unwrap_or(10);
        let after = after.unwrap_or(0);

        let records = sqlx::query_as!(
            RecipeDB,
            r#"
            SELECT id, author, title, banner_image_url, servings,
                   prep_time, cook_time, rest_time, difficulty,
                   vote_count, vote_average
            FROM recipes
            ORDER BY id
            LIMIT ? OFFSET ?
            "#,
            first,
            after
        )
        .fetch_all(db)
        .await?;

        Ok(records)
    }
}
