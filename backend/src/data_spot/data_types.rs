use async_graphql::{SimpleObject, Enum, Object};
//use chrono::{DateTime, Utc};
/*
export interface Indication {
  label: string;
  value: string;
}

export interface RecipeStep {
  text: string;
  imageUrl?: string;
}
*/
/*
export interface Ingredient {
  name: string;
  quantity: number;
  unit: string;
}
*/
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
pub struct Author {
    pub id: String,
    pub name: String,
    pub avatar_url: String,
}

/*
export interface Recipe {
  id: string;
  title: string;
  bannerUrl: string;
  servings: number;
  ingredients: Ingredient[];
  indications: Indication[];
  steps: RecipeStep[];
  author: {
    id: string;
    name: string;
    avatarUrl: string;
  };
  vote_average: number;
}
*/
pub struct Recipe {
    pub id: i64,
    pub author: Author,
    pub title: String,
    pub banner_url: Option<String>,
    pub servings: i32,
    pub ingredients: Vec<RecipeIngredient>,
    pub indications: Indication,
    //pub introduction: String,
    pub steps: Vec<Step>,
    //pub conclusion: String,
    //pub created_at: DateTime<Utc>,
    //pub tags: Vec<Tag>,
    pub vote_average: f64,
    pub votes: i32,
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

#[derive(SimpleObject, Clone)]
pub struct Indication {
    pub label: String,
    pub value: String,
}

#[Object]
impl Recipe {
    async fn id(&self) -> i64 {
        self.id
    }

    async fn author(&self) -> Author {
        self.author.clone()
    }

    async fn title(&self) -> String {
        self.title.clone()
    }

    async fn banner_url(&self) -> Option<String> {
        self.banner_url.clone()
    }

    async fn servings(&self) -> i32 {
        self.servings
    }

    async fn ingredients(&self) -> Vec<RecipeIngredient> {
        self.ingredients.clone()
    }

    async fn indications(&self) -> Indication {
        self.indications.clone()
    }

    // async fn introduction(&self) -> String {
    //     self.introduction.clone()
    // }

    async fn steps(&self) -> Vec<Step> {
        self.steps.clone()
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

    async fn vote_average(&self) -> f64 {
        self.vote_average
    }

    async fn votes(&self) -> i32 {
        self.votes
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
