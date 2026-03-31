use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use schemars::schema_for;
use tower_http::cors::CorsLayer;

use crate::adapters::validation::SharedLogEntries;
use crate::domain::log_record::TradeLogRecord;
use crate::domain::metrics::{AgentMetrics, MetricsSnapshot};
use crate::domain::model::Decision;

#[derive(Clone)]
pub struct AppState {
    pub entries: SharedLogEntries,
    pub metrics: AgentMetrics,
}

async fn get_logs(State(state): State<AppState>) -> Json<Vec<TradeLogRecord>> {
    let g = state.entries.lock().expect("log mutex poisoned");
    Json(g.clone())
}

async fn get_metrics(State(state): State<AppState>) -> Json<MetricsSnapshot> {
    Json(state.metrics.snapshot())
}

async fn get_decision_schema() -> Json<serde_json::Value> {
    let schema = schema_for!(Decision);
    Json(serde_json::to_value(schema).unwrap_or_else(|_| serde_json::json!({})))
}

pub fn router(entries: SharedLogEntries, metrics: AgentMetrics) -> Router {
    let state = AppState { entries, metrics };
    Router::new()
        .route("/logs", get(get_logs))
        .route("/metrics", get(get_metrics))
        .route("/decision-schema", get(get_decision_schema))
        .with_state(state)
        .layer(CorsLayer::permissive())
}

pub async fn serve(router: Router, addr: std::net::SocketAddr) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
