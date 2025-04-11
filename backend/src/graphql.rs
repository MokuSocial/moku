use async_graphql::{Context, Object};

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, ctx: &Context<'_>) -> String {
        let word = "World".to_string();
        let name = ctx.data_opt::<String>().unwrap_or(&word);
        format!("Hello, {}!", name)
    }
}

