use axum::{
    middleware,
    routing::{get, put},
    Router,
};

use crate::{
    handlers::posts_handler::*,
    midlewares::posts_midleware::{after_action, before_action, round_action},
    state::AppState,
};

pub fn create_routes() -> Router {
    let app_state = AppState::default();
    Router::new()
        .route(
            "/posts",
            get(posts)
                .layer(middleware::from_fn(before_action))
                .post(create_post)
                .layer(middleware::from_fn(after_action)),
        )
        .route("/posts/{id}", put(update_post).delete(delete_post))
        .route("/posts/{id}/like", put(like_post).layer(middleware::from_fn(round_action)))
        .route("/posts/{id}/dislike", put(dislike_post))
        .with_state(app_state)
}
