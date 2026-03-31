use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use tower_http::cors::CorsLayer;

use crate::domain::log_record::TradeLogRecord;
use crate::adapters::validation::SharedLogEntries;

async fn get_logs(State(entries): State<SharedLogEntries>) -> Json<Vec<TradeLogRecord>> {
    let g = entries.lock().expect("log mutex poisoned");
    Json(g.clone())
}

pub fn router(entries: SharedLogEntries) -> Router {
    Router::new()
        .route("/logs", get(get_logs))
        .with_state(entries)
        .layer(CorsLayer::permissive())
}

pub async fn serve(router: Router, addr: std::net::SocketAddr) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
