use async_graphql::{Context, Object, SimpleObject};
use crate::data_spot::data_types::Recipe;

/*
export interface ApiResult {
  page: number;
  results: Recipe[];
  total_pages: number;
  total_results: number;
}

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

export interface Ingredient {
  name: string;
  quantity: number;
  unit: string;
}

export interface Indication {
  label: string;
  value: string;
}

export interface RecipeStep {
  text: string;
  imageUrl?: string;
}
*/
#[derive(SimpleObject)]
pub struct QueryRecipeResult {
    pub page: i32,
    pub results: Vec<Recipe>,
    pub total_pages: i32,
    pub total_results: i32,
}

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }

    async fn recipe(&self, ctx: &Context<'_>, id: i64) -> Option<Recipe> {
        None
    }

    async fn recipes(&self, ctx: &Context<'_>, url: Option<String>, id: i64) -> QueryRecipeResult {
      QueryRecipeResult {
        page: 1,
        results: vec![],
        total_pages: 1,
        total_results: 0,
      }
    }
}
