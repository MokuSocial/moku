use axum::{routing::get, Router};
use std::net::{IpAddr, SocketAddr};
use tokio;
use tower_http::cors::{CorsLayer};
use crate::db::DatabaseHandler;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQL};
use dotenv::dotenv;
use std::env;

use crate::graphql::{Query,mutation::Mutation};


//mod data_spot;
mod graphql;
mod db;
mod data_types;
mod auth;

#[tokio::main]
async fn main() {
    // Carica le variabili d'ambiente dal file .env
    dotenv().ok();

    let _port = env::var("PORT").unwrap_or("8080".to_string());
    let _host: IpAddr = env::var("HOST").unwrap_or("0.0.0.0".to_string()).parse().unwrap();
    // Inizializza il database
    let db = DatabaseHandler::new().await.expect("Failed to initialize database");

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db.clone())
        .finish();

    // Configura il router con una route di test
    let app = Router::new()
        .route_service( "/graphql", GraphQL::new(schema)).with_state(db)
        .route("/", get(|| async { "Hello, Axum!" }))
        .layer(CorsLayer::permissive());

    // Imposta l'indirizzo del server
    let addr = SocketAddr::from((_host, _port.parse().unwrap()));
    println!("Server in esecuzione su http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
