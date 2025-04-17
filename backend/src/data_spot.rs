/* Data Single Point of Truth (SPOT) */

pub mod db;
pub mod data_types;

//use data_types::{Ingredient, Step, Recipe, Tag};

pub async fn setup() -> Result<(), String> {
    // Inizializza il database
    db::setup().await

}
