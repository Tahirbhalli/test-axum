use crate::{
    handlers::{auth_handler::auth_router, posts_handler::posts_router},
    state::AppState,
};
use axum::Router;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .nest("/posts", posts_router(state.clone()))
        .nest("/users", auth_router(state.clone()))
}
