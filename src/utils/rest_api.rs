use crate::utils::bundles::retrieve_bundle_envelopes;
use axum::response::IntoResponse;
use axum::{extract::Path, response::Json};
use bundler::utils::core::tx_envelope_writer::TxEnvelopeWrapper;
use reqwest::{header, StatusCode};
use serde_json::{json, Value};

pub async fn get_status() -> Json<Value> {
    Json(json!({"status": "running"}))
}

pub async fn get_envelope(
    Path((bundle_txid, envelope_index)): Path<(String, u32)>,
) -> impl IntoResponse {
    let envelopes = retrieve_bundle_envelopes(bundle_txid).await.unwrap();
    let envelope_index = envelope_index as usize;

    if (envelopes.len() < envelope_index) {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "envelope index out of range"})),
        )
            .into_response();
    }

    let envelope: &TxEnvelopeWrapper = &envelopes[envelope_index];

    let input: String = match Some(envelope.clone().input) {
        Some(input) => input,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "No input data found in envelope" })),
            )
                .into_response()
        }
    };

    let content_type = envelope
        .tags
        .clone()
        .map(|tags| {
            tags.iter()
                .find(|tag| tag.name.to_lowercase() == "content-type")
                .map(|tag| tag.value.clone())
        })
        .flatten()
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let processed_data = match hex::decode(input.trim_start_matches("0x")) {
        Ok(data) => data,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Failed to process input data" })),
            )
                .into_response()
        }
    };

    (
        [
            (header::CONTENT_TYPE, content_type),
            (
                header::CACHE_CONTROL,
                "public, max-age=31536000".to_string(),
            ),
        ],
        processed_data,
    )
        .into_response()
}
