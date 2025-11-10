use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{Any, CorsLayer};

use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;

//mod data_spot;
mod graphql;
mod db;
mod data_types;


// Handler per GraphQL
async fn graphql_handler(req: GraphQLRequest) -> GraphQLResponse {
    let schema = Schema::build(graphql::Query, EmptyMutation, EmptySubscription)
        .data(db::setup().await.expect("Failed to set up database"))
        .finish();
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() {
    // Inizializza il database
    let setup = setup();
    //let pool = db::setup().await.expect("Failed to set up database");
    // Configura il router con una route di test
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/", get(|| async { "Hello, Axum!" }))
        .layer(CorsLayer::new().allow_origin(Any));

    // Imposta l'indirizzo del server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server in esecuzione su http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    // Avvia il server
    setup.await;
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn setup() {
}
