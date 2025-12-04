use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{Any, CorsLayer};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQL};

use crate::db::DatabaseHandler;
use crate::graphql::{Query,mutation::Mutation};

//mod data_spot;
mod graphql;
mod db;
mod data_types;
mod auth;

#[tokio::main]
async fn main() {
    // Inizializza il database
    let db = DatabaseHandler::new().await.expect("Failed to initialize database");

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db.clone())
        .finish();

    // Configura il router con una route di test
    let app = Router::new()
        .route_service( "/graphql", GraphQL::new(schema)).with_state(db)
        .route("/", get(|| async { "Hello, Axum!" }))
        .layer(CorsLayer::new().allow_origin(Any));

    // Imposta l'indirizzo del server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server in esecuzione su http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
