use crate::{
    handlers::{auth_handler::auth_router, posts_handler::posts_router},
    state::AppState,
};
use axum::Router;
use tower_http::trace::TraceLayer;

pub fn create_routes(state: AppState) -> Router {
    tracing_subscriber::fmt().with_env_filter("info").init();

    Router::new()
        .nest("/users", auth_router(state.clone()))
        .nest("/posts", posts_router(state.clone()))
        .layer(TraceLayer::new_for_http())
}
