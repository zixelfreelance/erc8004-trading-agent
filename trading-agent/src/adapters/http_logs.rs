use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use schemars::schema_for;
use tower_http::cors::CorsLayer;

use crate::adapters::validation::SharedLogEntries;
use crate::domain::log_record::TradeLogRecord;
use crate::domain::model::Decision;

async fn get_logs(State(entries): State<SharedLogEntries>) -> Json<Vec<TradeLogRecord>> {
    let g = entries.lock().expect("log mutex poisoned");
    Json(g.clone())
}

async fn get_decision_schema() -> Json<serde_json::Value> {
    let schema = schema_for!(Decision);
    Json(serde_json::to_value(schema).unwrap_or_else(|_| serde_json::json!({})))
}

pub fn router(entries: SharedLogEntries) -> Router {
    Router::new()
        .route("/logs", get(get_logs))
        .route("/decision-schema", get(get_decision_schema))
        .with_state(entries)
        .layer(CorsLayer::permissive())
}

pub async fn serve(router: Router, addr: std::net::SocketAddr) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
