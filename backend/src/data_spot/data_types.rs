use async_graphql::{SimpleObject, Enum, Object};

#[derive(SimpleObject, Clone)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
    pub identifier: String,
    pub wikidata: Option<String>,
    pub cost_per_unit: Option<f64>,
    pub unit: Option<UnitOfMeasure>,
}

#[derive(SimpleObject, Clone)]
pub struct RecipeIngredient {
    pub ingredient: Ingredient,
    pub quantity: f64,
    pub unit: Option<UnitOfMeasure>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UnitOfMeasure {
    Gram,
    Kilogram,
    Liter,
    Milliliter,
    Piece,
}

pub struct Recipe {
    pub id: i64,
    pub user_id: String,
    pub title: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub introduction: String,
    pub steps: Vec<Step>,
    pub conclusion: String,
    pub created_at: String,
    pub tags: Vec<Tag>,
}

#[derive(SimpleObject, Clone)]
pub struct Step {
    pub step_number: i32,
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(SimpleObject, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub autotag: Option<bool>,
}

#[Object]
impl Recipe {
    async fn id(&self) -> i64 {
        self.id
    }

    async fn user_id(&self) -> String {
        self.user_id.clone()
    }

    async fn title(&self) -> String {
        self.title.clone()
    }

    async fn ingredients(&self) -> Vec<RecipeIngredient> {
        self.ingredients.clone()
    }

    async fn introduction(&self) -> String {
        self.introduction.clone()
    }

    async fn steps(&self) -> Vec<Step> {
        self.steps.clone()
    }

    async fn conclusion(&self) -> String {
        self.conclusion.clone()
    }

    async fn created_at(&self) -> String {
        self.created_at.clone()
    }

    async fn tags(&self) -> Vec<Tag> {
        self.tags.clone()
    }
}
