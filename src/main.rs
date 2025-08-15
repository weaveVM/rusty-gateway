use crate::utils::rest_api::{get_envelope, get_status};
use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub mod utils;

#[tokio::main]
async fn main() {
    // server routes
    let router = Router::new()
        .route("/", get(get_status))
        .route("/bundle/:bundle_txid/:envelope_index", get(get_envelope));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on http://0.0.0.0:8000");
    
    axum::serve(listener, router).await.unwrap();
}
