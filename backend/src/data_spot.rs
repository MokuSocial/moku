/* Data Single Point of Truth (SPOT) */

mod db;

pub async fn setup() {
    // Inizializza il database
    let pool = db::connect_db().await;
    db::initialize_db(&pool).await.unwrap();
}
