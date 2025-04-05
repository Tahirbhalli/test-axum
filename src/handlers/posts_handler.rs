use crate::{
    dto::jwt_dto::Claims, entities::post, error::AppError,
    midlewares::auth_midleware::authenticate_user, state::AppState,
};
use axum::{extract::State, middleware, routing::get, Extension, Json, Router};
use sea_orm::EntityTrait;
use serde_json::json;

pub fn posts_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_posts))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            authenticate_user,
        ))
        .with_state(state)
}

async fn get_posts(
    State(app_state): State<AppState>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<serde_json::Value>, AppError> {
    let posts = post::Entity::find().all(&app_state.db).await.unwrap();
    Ok(Json(json!({"posts": posts})))
}
