use crate::utils::rest_api::{get_envelope, get_status};
use axum::{routing::get, Router};

pub mod utils;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // server routes
    let router = Router::new()
        .route("/", get(get_status))
        .route("/bundle/:bundle_txid/:envelope_index", get(get_envelope));

    Ok(router.into())
}
