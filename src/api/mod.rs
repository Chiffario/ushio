use crate::state::{AppState, SharedState};
use apply::Apply as _;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, Debug)]
struct ClientDistributionQuery {
    from: OffsetDateTime,
    to: OffsetDateTime,
}

#[derive(Serialize)]
struct ClientDistributionResponse {
    stable: u32,
    lazer: u32,
}

/// Describes GET /api/scores/distribution?from={}&to={}
///
/// Returns client distribution across all scores within the specified range
async fn get_score_distribution_in_range(
    Query(query): Query<ClientDistributionQuery>,
    State(state): State<SharedState>,
) -> Json<ClientDistributionResponse> {
    todo!();
}

#[derive(Deserialize, Serialize, Debug)]
struct UserIdDistributionQuery {
    from: OffsetDateTime,
    to: OffsetDateTime,
    // TODO: bucket_size: ???
    // it has to be discrete but idk what kind of precision to provide
}

#[derive(Serialize, Debug)]
struct UserIdDistributionEntry {
    stable: u32,
    lazer: u32,
    bucket: String,
}

/// Describes GET /api/scores/distribution/user_id?from={}&to={}
async fn get_score_distribution_by_user_id(
    Query(query): Query<UserIdDistributionQuery>,
    State(state): State<SharedState>,
) -> Json<Vec<UserIdDistributionEntry>> {
    todo!();
}

async fn get_daily_unique_per_client(
    State(state): State<SharedState>,
) -> Result<Json<Vec<UserIdDistributionEntry>>, StatusCode> {
    state
        .lock()
        .await
        .database()
        .get_daily_unique_users()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|bucket| UserIdDistributionEntry {
            stable: bucket.stable as u32,
            lazer: bucket.lazer as u32,
            bucket: bucket.bucket_floor,
        })
        .collect::<Vec<_>>()
        .apply(Json)
        .apply(Ok)
}

pub(crate) fn router() -> Router<SharedState> {
    Router::new()
        .route("/distribution/daily", get(get_daily_unique_per_client))
        .route("/distribution", get(get_score_distribution_in_range))
        .route(
            "/distribution/user_id",
            get(get_score_distribution_by_user_id),
        )
}
