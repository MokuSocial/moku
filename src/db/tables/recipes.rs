use crate::data_types::{Author, Indication, Recipe};


#[derive(sqlx::FromRow)]
pub struct RecipeDB {
    pub id : i64,
    pub author : String,
    pub title : String,
    //introduction : String,
    //conclusion : String,
    pub prep_time: u32,
    pub cook_time: u32,
    pub rest_time: Option<u32>,
    pub difficulty: String,
    pub banner_url : Option<String>,
    pub servings : u16,
    pub vote_count : u32,
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
            banner_url: recipe.banner_url,
            prep_time: recipe.indication.prep_time,
            cook_time: recipe.indication.cook_time,
            rest_time: recipe.indication.rest_time,
            difficulty: recipe.indication.difficulty,
            vote_count: recipe.votes,
            vote_average: recipe.vote_average,
            servings: recipe.servings,
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
        banner_url: rec_db.banner_url,
        indication: Indication {
            prep_time: rec_db.prep_time,
            cook_time: rec_db.cook_time,
            rest_time: rec_db.rest_time,
            difficulty: rec_db.difficulty,
        },
        votes: rec_db.vote_count,
        vote_average: rec_db.vote_average,
        servings: rec_db.servings,
        }
    }
}

impl RecipeDB {
    pub async fn get(
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
                        prep_time: 0,
                        cook_time: 0,
                        rest_time: None,
                        difficulty: String::new(),
                        servings: record.servings as u16,
                        vote_count: record.vote_count as u32,
                        vote_average: record.vote_average as f64,
                    })
    }

    pub async fn get_recipe_with_indications(
        db: &sqlx::SqlitePool,
        id: i64
    ) -> Result<RecipeDB, sqlx::Error> {
        let record = sqlx::query!(
            "SELECT id, author, title, banner_image_url, servings, prep_time, cook_time, rest_time, difficulty, vote_count, vote_average FROM recipes WHERE id = ?",
            id
        )
        .fetch_optional(db)
        .await?.map_or(Err(sqlx::Error::RowNotFound), |record| Ok(record))?;

        Ok(RecipeDB {
                        id: record.id,
                        author: record.author,
                        title: record.title,
                        banner_url: record.banner_image_url,
                        prep_time: record.prep_time as u32,
                        cook_time: record.cook_time as u32,
                        rest_time: record.rest_time.map(|t| t as u32),
                        difficulty: record.difficulty,
                        servings: record.servings as u16,
                        vote_count: record.vote_count as u32,
                        vote_average: record.vote_average as f64,
                    })
    }

}
