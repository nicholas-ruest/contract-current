use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use cc_auth::principal;
use cc_config::Config;
use cc_errors::CurrentError;
use cc_orchestrator::Orchestrator;
use cc_types::{ApiError, CreateMigrationDecision, Health};
use std::{sync::Arc, time::Duration};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
#[derive(Clone)]
struct App {
    orch: Arc<Orchestrator>,
    token: String,
}
fn fail(e: CurrentError, c: String) -> Response {
    let (status, code) = match e {
        CurrentError::Invalid(_) => (StatusCode::UNPROCESSABLE_ENTITY, "invalid"),
        CurrentError::Denied(_) => (StatusCode::FORBIDDEN, "denied"),
        CurrentError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
        CurrentError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
        _ => (StatusCode::SERVICE_UNAVAILABLE, "dependency"),
    };
    (
        status,
        Json(ApiError {
            code: code.into(),
            message: e.to_string(),
            correlation_id: c,
        }),
    )
        .into_response()
}
#[allow(clippy::result_large_err)]
fn ctx(h: &HeaderMap, s: &App) -> Result<String, Response> {
    let c = cc_middleware::correlation(h.get("x-correlation-id").and_then(|x| x.to_str().ok()));
    cc_middleware::authorize(
        h.get("authorization").and_then(|x| x.to_str().ok()),
        &s.token,
    )
    .map_err(|e| fail(e, c.clone()))?;
    let _p = principal(&s.token);
    Ok(c)
}
async fn health() -> Json<Health> {
    Json(Health {
        status: "ready".into(),
        persistence: "rvf".into(),
        policy: "cedar-policy".into(),
    })
}
async fn create(
    State(s): State<App>,
    h: HeaderMap,
    Json(r): Json<CreateMigrationDecision>,
) -> Response {
    let c = match ctx(&h, &s) {
        Ok(x) => x,
        Err(r) => return r,
    };
    match s.orch.create(&r.revision, &r.change_class, c.clone()) {
        Ok(m) => (StatusCode::CREATED, Json(m)).into_response(),
        Err(e) => fail(e, c),
    }
}
async fn list(State(s): State<App>, h: HeaderMap) -> Response {
    let c = match ctx(&h, &s) {
        Ok(x) => x,
        Err(r) => return r,
    };
    match s.orch.list() {
        Ok(v) => Json(v).into_response(),
        Err(e) => fail(e, c),
    }
}
async fn get_one(State(s): State<App>, h: HeaderMap, Path(id): Path<String>) -> Response {
    let c = match ctx(&h, &s) {
        Ok(x) => x,
        Err(r) => return r,
    };
    match s.orch.get(&id) {
        Ok(v) => Json(v).into_response(),
        Err(e) => fail(e, c),
    }
}
pub async fn serve(config: Config) -> cc_errors::Result<()> {
    let orch = Arc::new(Orchestrator::open(config.data_path)?);
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(health))
        .route("/v1/changes", post(create).get(list))
        .route("/v1/changes/{id}", get(get_one))
        .layer(CorsLayer::permissive())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(App {
            orch,
            token: config.token,
        });
    let listener = tokio::net::TcpListener::bind(&config.bind)
        .await
        .map_err(|e| CurrentError::Dependency(e.to_string()))?;
    axum::serve(listener, app)
        .await
        .map_err(|e| CurrentError::Dependency(e.to_string()))
}
#[cfg(test)]
mod tests {
    #[test]
    fn versioned_contract() {
        assert!(include_str!("../../../openapi.yaml").contains("/v1/changes"))
    }
}
