use async_graphql::{ComplexObject, Enum, Context, SimpleObject};

use crate::db;
//use chrono::{DateTime, Utc};

#[derive(SimpleObject, Clone)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
    // pub identifier: String,
    // pub wikidata: Option<String>,
    //pub cost_per_unit: Option<f64>,
    pub unit: Option<UnitOfMeasure>,
}

#[derive(SimpleObject, Clone)]
pub struct RecipeIngredient {
    pub id: i64,
    pub name: String,
    pub unit: Option<String>,
    pub quantity: f64,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UnitOfMeasure {
    Gram,
    Kilogram,
    Liter,
    Milliliter,
    Piece,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Recipe {
    pub id: i64,
    pub author: Author,
    pub title: String,
    pub banner_url: Option<String>,
    pub servings: u16,
    pub indication: Indication,
    //pub introduction: String,
    //pub conclusion: String,
    //pub created_at: DateTime<Utc>,
    //pub tags: Vec<Tag>,
    pub vote_average: f64,
    pub votes: u32,
}

#[derive(SimpleObject, Clone)]
pub struct Step {
    pub step_number: u32,
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Author {
    pub username: String,
}

#[derive(SimpleObject, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub autotag: Option<bool>,
}

#[derive(SimpleObject, Clone)]
pub struct Indication {
    /*prepTime: number;
    cookTime: number;
    restTime?: number;
    difficulty: 'easy' | 'medium' | 'hard';*/
    pub prep_time: u32,
    pub cook_time: u32,
    pub rest_time: Option<u32>,
    pub difficulty: String,
}

#[ComplexObject]
impl Recipe {

    async fn ingredients(&self, ctx: &Context<'_>) -> Vec<RecipeIngredient> {
        db::get_recipe_ingredients(&ctx.data_unchecked::<sqlx::SqlitePool>(), self.id).await.unwrap_or_default()
    }

    async fn steps(&self, ctx: &Context<'_>) -> Vec<Step> {
        db::get_steps(&ctx.data_unchecked::<sqlx::SqlitePool>(), self.id).await.unwrap_or_default()
    }

}

#[ComplexObject]
impl Author { //TODO
    async fn name(&self) -> String {
        self.username.clone()
    }
    async fn avatar_url(&self) -> String {
        "https://picsum.photos/256".to_string()
    }
}

impl UnitOfMeasure {
    pub fn from_str(unit: &str) -> Option<Self> {
        match unit {
            "g" => Some(UnitOfMeasure::Gram),
            "kg" => Some(UnitOfMeasure::Kilogram),
            "l" => Some(UnitOfMeasure::Liter),
            "ml" => Some(UnitOfMeasure::Milliliter),
            "piece" => Some(UnitOfMeasure::Piece),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            UnitOfMeasure::Gram => "g",
            UnitOfMeasure::Kilogram => "kg",
            UnitOfMeasure::Liter => "l",
            UnitOfMeasure::Milliliter => "ml",
            UnitOfMeasure::Piece => "piece",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}
