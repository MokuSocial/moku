use async_graphql::{Context, Object, SimpleObject};
use crate::{data_types::Recipe, db};

/*

*/
#[derive(SimpleObject)]
pub struct QueryRecipeResult {
    //pub page: i32,
    pub results: Vec<Recipe>,
    //pub total_pages: i32,
    pub total_results: i32,
}

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }

    async fn recipe(&self, ctx: &Context<'_>, id: i64) -> Option<Recipe> {
      if ctx.look_ahead().field("indications").exists() {
        println!("Fetching recipe with indications");
        db::get_recipe_with_indications(&ctx.data_unchecked::<sqlx::SqlitePool>(), id).await.ok()
      } else {
        println!("Fetching recipe without indications");
        db::get_recipe(&ctx.data_unchecked::<sqlx::SqlitePool>(), id).await.ok()
      }
    }

    async fn recipes(&self, ctx: &Context<'_>) -> QueryRecipeResult {
      let recs = db::get_recipes(&ctx.data_unchecked::<sqlx::SqlitePool>()).await.unwrap_or_default();
      let recs_len = recs.len();
      QueryRecipeResult {
        //page: 1,
        results: recs,
        //total_pages: 1,
        total_results: recs_len as i32,
      }

    }
}
