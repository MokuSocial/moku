use async_graphql::{ComplexObject, Enum, Object, SimpleObject};
//use chrono::{DateTime, Utc};

#[derive(SimpleObject, Clone)]
pub struct Ingredient {
    id: i64,
    pub name: String,
    // pub identifier: String,
    // pub wikidata: Option<String>,
    //pub cost_per_unit: Option<f64>,
    unit: Option<UnitOfMeasure>,
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

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Recipe {
    pub id: i64,
    pub author: Author,
    pub title: String,
    pub banner_url: Option<String>,
    pub servings: u16,
    //pub introduction: String,
    //pub conclusion: String,
    //pub created_at: DateTime<Utc>,
    //pub tags: Vec<Tag>,
    pub vote_average: f64,
    pub votes: u32,
}

#[derive(SimpleObject, Clone)]
pub struct Step {
    pub step_number: i32,
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
    pub label: String,
    pub value: String,
}

#[ComplexObject]
impl Recipe {

    async fn ingredients(&self) -> Vec<RecipeIngredient> {
        vec![] // Placeholder implementation
    }

    async fn indications(&self) -> Indication {
        Indication { // Placeholder implementation
            label: "Unimplemented".to_string(),
            value: "Pleas wait our slow devs".to_string(),
        }
    }

    // async fn introduction(&self) -> String {
    //     self.introduction.clone()
    // }

    async fn steps(&self) -> Vec<Step> {
        vec![] // Placeholder implementation
    }

    // async fn conclusion(&self) -> String {
    //     self.conclusion.clone()
    // }

    // async fn created_at(&self) -> String {
    //     self.created_at.to_rfc3339()
    // }

    // async fn tags(&self) -> Vec<Tag> {
    //     self.tags.clone()
    // }
}

#[ComplexObject]
impl Author {
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
