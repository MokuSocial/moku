use sqlx::sqlite::SqliteQueryResult;
use crate::data_types::Step;

#[derive(sqlx::FromRow)]
pub struct RecipeStepDB {
    pub recipe_id : i64,
    pub step_number : i64,
    pub description : String,
    pub image_url : Option<String>,
}

impl From<RecipeStepDB> for Step {
    fn from(step_db: RecipeStepDB) -> Self {
        Self {
            step_number: step_db.step_number as u32,
            description: step_db.description,
            image_url: step_db.image_url,
        }
    }
}

impl From<&RecipeStepDB> for Step {
    fn from(step_db: &RecipeStepDB) -> Self {
        Self {
            step_number: step_db.step_number as u32,
            description: step_db.description.clone(),
            image_url: step_db.image_url.clone(),
        }
    }
}

impl RecipeStepDB {

    pub fn from(step: Step, recipe_id: i64) -> Self {
        Self {
            recipe_id,
            step_number: step.step_number as i64,
            description: step.description,
            image_url: step.image_url,
        }
    }

/*
    pub async fn add(
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
    }*/

    pub async fn get(
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

    pub async fn gets(
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
    /*
    pub async fn delete(
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

    pub async fn update(
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
    }*/
}
