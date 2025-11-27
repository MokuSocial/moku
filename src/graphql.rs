use async_graphql::{Context, Object, SimpleObject, Result, connection::{EmptyFields, Connection, Edge, query}};
use crate::{data_types::Recipe, db::DatabaseHandler};

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
      let db = ctx.data::<DatabaseHandler>().unwrap();

      if ctx.look_ahead().field("indications").exists() {
        println!("Fetching recipe with indications");
        db.get_recipe_with_indications(id).await.ok()
      } else {
        println!("Fetching recipe without indications");
        db.get_recipe(id).await.ok()
      }
    }

    async fn recipes(&self, ctx: &Context<'_>,
      after: Option<String>,
      //before: Option<String>,
      first: Option<i32>,
      //last: Option<i32>
      ) -> Result<Connection<i64, Recipe, EmptyFields, EmptyFields>> {
      let db = ctx.data::<DatabaseHandler>().unwrap();
      let before: Option<String> = None;
      let last: Option<i32> = None;
      query(after, before, first, last, |after, before, first, last| async move {
        if before.is_some() || last.is_some() {
            return Err(async_graphql::Error::new("Backward pagination is not supported, yet"));
        }
        let recs = db.get_recipes(first.map(|e| e as i64), after).await.unwrap_or_default();

        let mut connection = Connection::new(false, false);

        connection.edges.extend(
            recs.into_iter().enumerate().map(|(i, rec)| {
                Edge::new(i as i64, rec)
            })
        );
        Ok::<_,async_graphql::Error>(connection)
      }).await
    }

    async fn numbers(&self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, i32, EmptyFields, EmptyFields>> {
        query(after, before, first, last, |after, before, first, last| async move {
            let mut start = after.map(|after| after + 1).unwrap_or(0);
            let mut end = before.unwrap_or(10000);
            if let Some(first) = first {
                end = (start + first).min(end);
            }
            if let Some(last) = last {
                start = if last > end - start {
                     end
                } else {
                    end - last
                };
            }
            let mut connection = Connection::new(start > 0, end < 10000);
            connection.edges.extend(
                (start..end).into_iter().map(|n|
                    Edge::with_additional_fields(n, n as i32, EmptyFields)
            ));
            Ok::<_, async_graphql::Error>(connection)
        }).await
    }
}
