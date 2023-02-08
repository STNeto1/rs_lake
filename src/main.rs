use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use client::queries::ADD_LOG_QUERY;
use entry::{get_data_type, Duration, Entry};
use scylla::Session;
use serde_json::Value;
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod client;
mod entry;

struct AppState {
    client: Session,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let uri = "127.0.0.1:9042".to_owned();
    let client = rs_lake::create_scylla_client(&uri)
        .await
        .expect("failed to create scylla session");

    let app_state = Arc::new(AppState { client });

    let app = Router::new()
        .route("/:topic", post(handle_data_input))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_data_input(
    State(state): State<Arc<AppState>>,
    Path(topic): Path<String>,
    Json(input): Json<Value>,
) -> impl IntoResponse {
    let entry = Entry {
        topic,
        timestamp: Duration::now(),
        data_type: get_data_type(&input),
        data: input.to_string(),
    };

    return match state.client.query(ADD_LOG_QUERY, entry).await.map(|_| ()) {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::debug!("listening on {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
}
