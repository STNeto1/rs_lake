use std::net::SocketAddr;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use chrono::Local;
use lake_data::{Data, Entry};
use serde_json::Value;

mod lake_data;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:topic", post(handle_data_input));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_data_input(
    Path(topic): Path<String>,
    Json(input): Json<Value>,
) -> impl IntoResponse {
    let data = Data::new_from_input(&input);

    let entry = Entry {
        topic,
        timestamp: Local::now().to_rfc3339(),
        data,
    };

    return StatusCode::OK;
}
