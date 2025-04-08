use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{Any, CorsLayer};

mod data_spot;

#[tokio::main]
async fn main() {
    // Inizializza il database
    let setup = setup_db();
    // Configura il router con una route di test
    let app = Router::new()
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

async fn setup_db() {
    // Inizializza il database
    let pool = data_spot::db::connect_db().await;
    data_spot::db::initialize_db(&pool).await.unwrap();
}
