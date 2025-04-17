use async_graphql::{Context, Object};
use crate::data_spot::data_types::Recipe;

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }

    async fn recipe(&self, ctx: &Context<'_>, id: i64) -> Option<Recipe> {
        None
    }

    async fn recipes(&self, ctx: &Context<'_>, url: Option<String>, id: i64) -> Vec<Recipe> {
        vec![]
    }
}
