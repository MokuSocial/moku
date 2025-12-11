// File: db.rs

mod handler;
mod tables;

use crate::data_types::{Recipe, Step, RecipeIngredient};

use sqlx::SqlitePool;

pub trait FromDB<T> {
    fn from_db(db: &SqlitePool, item: T) -> Result<Self, String>
    where
        Self: Sized;
    
}

#[derive(Clone)]
pub struct DatabaseHandler {
    total_recipes: i64,
    pool: SqlitePool,
}


impl DatabaseHandler {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = handler::connect().await;
        let total_recipes = tables::recipes::RecipeDB::count(&pool).await.unwrap_or(0);

        let ret = Self { 
            total_recipes,
            pool
        };
        handler::initialize(&ret.pool).await?;
        Ok(ret)
    }

    pub fn get_total_recipes(self: &Self) -> i64 {
        self.total_recipes
    }

    /*
    pub async fn add_recipe(self: &mut Self, recipe: Recipe) -> Result<i64, String> {
        (*self).total_recipes += 1;
        Ok(recipe.id)
    }*/
    
    pub async fn get_recipe(self: &Self, id: i64) -> Result<Recipe, String> {
        let rec_db = tables::recipes::RecipeDB::get(&self.pool, id).await.map_err(|e| e.to_string())?;
        Ok(Recipe::from(rec_db))
    }

    pub async fn get_recipe_with_indications(self: &Self, id: i64) -> Result<Recipe, String> {
        let rec_db = tables::recipes::RecipeDB::get_recipe_with_indications(&self.pool, id).await.map_err(|e| e.to_string())?;
        Ok(Recipe::from(rec_db))
    }

    pub async fn get_steps(self: &Self, recipe_id: i64) -> Result<Vec<Step>, String> {
        let steps_db = tables::recipe_steps::RecipeStepDB::gets(&self.pool, recipe_id).await.map_err(|e| e.to_string())?;
        let steps: Vec<Step> = steps_db.into_iter().map(|s| Step::from(s)).collect();
        Ok(steps)
    }

    pub async fn get_recipe_ingredients(self: &Self, recipe_id: i64) -> Result<Vec<crate::data_types::RecipeIngredient>, String> {
        let ri_db = tables::recipe_ingredients::RecipeIngredientDB::gets_by_recepie_id(&self.pool, recipe_id).await.map_err(|e| e.to_string())?;
        let ingredients: Vec<RecipeIngredient> = ri_db.into_iter().map(|ri| crate::data_types::RecipeIngredient::from(ri)).collect();
        Ok(ingredients)
    }

    pub async fn get_recipes(self: &Self, first: Option<usize>, after: Option<i64>) -> Result<Vec<Recipe>, String> {
        let recs_db = tables::recipes::RecipeDB::gets(&self.pool,first.map(|e| e as i64),after).await.map_err(|e| e.to_string())?;
        let recipes: Vec<Recipe> = recs_db.into_iter().map(|r| Recipe::from(r)).collect();
        Ok(recipes)
    }
}
/*pub async fn get_ingredient(db: &SqlitePool, id: i64) -> Result<Ingredient, String> {
    let ing_db = tables::ingredients::IngredientDB::get(&db.pool, id).await.map_err(|e| e.to_string())?;

    Ok(Ingredient::from(ing_db))
}*/
