use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::state::{AppState, Post};
pub async fn posts(State(state): State<AppState>) -> impl IntoResponse {
    let posts = state.posts.lock().await;
    Json(posts.clone())
}
pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<Post>,
) -> impl IntoResponse {
    let mut posts = state.posts.lock().await;
    let post = Post {
        id: payload.id,
        msg: payload.msg,
        likes: payload.likes,
    };
    posts.push(post.clone());
    (StatusCode::CREATED, Json(post))
}
pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<usize>,
    Json(payload): Json<Post>,
) -> impl IntoResponse {
    let mut posts = state.posts.lock().await;
    if let Some(post) = posts.iter_mut().find(|post| post.id == id) {
        post.msg = payload.msg;
        post.likes = payload.likes;
    }
    StatusCode::OK
}

pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    let mut posts = state.posts.lock().await;

    let post_index = posts.iter().position(|post| post.id == id).unwrap();
    posts.remove(post_index);
    StatusCode::OK
}

pub async fn like_post(State(state): State<AppState>, Path(id): Path<usize>) -> impl IntoResponse {
    let mut posts = state.posts.lock().await;
    let post = posts.iter_mut().find(|post| post.id == id).unwrap();
    post.likes += 1;
    StatusCode::OK
}

pub async fn dislike_post(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    let mut posts = state.posts.lock().await;
    let post = posts.iter_mut().find(|post| post.id == id).unwrap();
    post.likes -= 1;
    StatusCode::OK
}
