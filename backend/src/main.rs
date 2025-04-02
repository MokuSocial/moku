use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // Configura il router con una route di test
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }))
        .layer(CorsLayer::new().allow_origin(Any));

    // Imposta l'indirizzo del server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server in esecuzione su http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    // Avvia il server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

