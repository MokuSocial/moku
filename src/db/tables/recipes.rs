use crate::data_types::Recipe;


#[derive(sqlx::FromRow)]
pub struct RecipeDB {
    pub id : i64,
    pub author : String,
    pub title : String,
    //introduction : String,
    //conclusion : String,
    pub banner_url : Option<String>,
    pub servings : u16,
    pub vote_count : u32,
    pub vote_average : f64,
    //pub created_at : i64,
    //pub last_updated : Option<i64>,
}

/*
impl RecipeDB {
    pub fn new(id: i64, author: String, title: String, introduction: String, conclusion: String, created_at: i64) -> Self {
        Self {
            id,
            author,
            title,
            //introduction,
            //conclusion,
            banner_url: None,
            created_at,
            last_updated: None,
        }
    }
}
*/

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
            author: recipe.author.username,
            title: recipe.title,
            //introduction: recipe.introduction,
            //conclusion: recipe.conclusion,
            banner_url: recipe.banner_url,
            //created_at: recipe.created_at.timestamp(),
            //last_updated: None, // This can be set later if needed
            vote_count: recipe.votes,
            vote_average: recipe.vote_average,
            servings: recipe.servings,
        }
    }
}

pub async fn get_recipe(
    db: &sqlx::SqlitePool,
    id: i64
) -> Result<RecipeDB, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT id, author, title, banner_image_url, servings, vote_count, vote_average FROM recipes WHERE id = ?",
        id
    )
    .fetch_optional(db)
    .await?.map_or(Err(sqlx::Error::RowNotFound), |record| Ok(record))?;

    Ok(RecipeDB {
                    id: record.id,
                    author: record.author,
                    title: record.title,
                    banner_url: record.banner_image_url,
                    servings: record.servings as u16,
                    vote_count: record.vote_count as u32,
                    vote_average: record.vote_average as f64,
                })
}
/*
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
*/
