use sqlx::{sqlite::SqliteQueryResult, Pool, Sqlite};
use crate::data_spot::{data_types::Recipe, db::{tables::{recipe_ingredients::{self, get_ingredients_by_recepie}, recipe_steps, recipe_tags, tags}, FromDB}};
use super::recipe_ingredients::get_recipe_ingredients;
use crate::data_spot::data_types::RecipeIngredient;


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

impl From<&Recipe> for RecipeDB{
    fn from(value: &Recipe) -> Self {
        /*
        Self {
            id: value.id,
            user_id: value.user_id,
            title: value.title,
            introduction: value.introduction,
            conclusion: value.conclusion,
            created_at: value.created_at.timestamp(),
            last_updated: None, // This can be set later if needed
        }*/
        value.to_owned().into()
    }
    
}

impl From<Recipe> for RecipeDB {
    fn from(recipe: Recipe) -> Self {
        Self {
            id: recipe.id,
            user_id: recipe.user_id,
            title: recipe.title,
            introduction: recipe.introduction,
            conclusion: recipe.conclusion,
            created_at: recipe.created_at.timestamp(),
            last_updated: None, // This can be set later if needed
        }
    }
}
/*

impl FromDB<RecipeDB> for Recipe {
    async fn from_db(db: &Pool<Sqlite>, recipe_db: RecipeDB) -> Result<Self, String> {
        let recipe_ingredients = get_recipe_ingredients(db, recipe_db.id).await
            .map_err(|e| e.to_string())?;
        
        let ingredients = recipe_ingredients.into_iter()
            .map(|ri| ri.into())
            .collect();

        let steps = recipe_steps::get_recipe_steps(db, recipe_db.id)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|step| step.into())
            .collect();

        let tags = recipe_tags::get_recipe_tags(db, recipe_db.id)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .filter_map(|tag| tags::get_tag(db, tag.tag_id).await.ok().flatten())
            .map(|t| t.into())
            .collect();

        Ok(Self {
            id: recipe_db.id,
            user_id: recipe_db.user_id,
            title: recipe_db.title,
            ingredients,
            introduction: recipe_db.introduction,
            conclusion: recipe_db.conclusion,
            created_at: chrono::DateTime::from_timestamp(recipe_db.created_at, 0)
                .unwrap(), // Convert i64 to DateTime
            steps,
            tags,
        })
    }
    
}*/

/*
=======
impl From<&Recipe> for RecipeDB {
    fn from(recipe: &Recipe) -> Self {
        Self {
            id: recipe.id,
            user_id: recipe.user_id.clone(),
            title: recipe.title.clone(),
            introduction: recipe.introduction.clone(),
            conclusion: recipe.conclusion.clone(),
            created_at: recipe.created_at.timestamp(),
            last_updated: None, // This can be set later if needed
        }
    }   
}

>>>>>>> Stashed changes
impl Recipe {
    pub async fn try_from_db(db : &Pool<Sqlite>, recipe_db: RecipeDB) -> Result<Self, std::error::Error> {
        let mut recipe_ingredients = vec![];
        let ris = recipe_ingredients::get_recipe_ingredients(db, recipe_db.id).await?;
        for ingredient in ris {
            let recipe_ingredient = RecipeIngredient::from_db(db, ingredient).await?;
            recipe_ingredients.push(recipe_ingredient);
        }
        
        Ok(Self {
                    id: recipe_db.id,
                    user_id: recipe_db.user_id,
                    title: recipe_db.title,
                    ingredients: recipe_ingredients,
                    introduction: recipe_db.introduction,
                    conclusion: recipe_db.conclusion,
                    created_at: chrono::DateTime::from_timestamp(recipe_db.created_at, 0)
                        .unwrap(), // Convert i64 to DateTime
                    steps: recipe_steps::get_recipe_steps(db, recipe_db.id)
                        .await?
                        .into_iter().map(|step| step.into()).collect(),
                    tags: {
                        let mut tags_v = vec![];
                        recipe_tags::get_recipe_tags(db, recipe_db.id)
                            .await?
                            .into_iter()
                            .for_each(|tag| tags::get_tag(db, tag.tag_id)
                                .await?
                                .map(|t| tags_v.push(t.into())))?;
                        // TODO add autotag
                        tags
                    }
                })
    }
}*/

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
) -> Result<RecipeDB, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT id, user_id, title, introduction, conclusion, created_at, last_updated FROM recipes WHERE id = ?",
        id
    )
    .fetch_optional(db)
    .await?.map_or(Err(sqlx::Error::RowNotFound), |record| Ok(record))?;

    Ok(RecipeDB {
                    id: record.id,
                    user_id: record.user_id,
                    title: record.title,
                    introduction: record.introduction,
                    conclusion: record.conclusion,
                    created_at: record.created_at,
                    last_updated: record.last_updated,
                })
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

