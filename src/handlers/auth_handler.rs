use axum::{extract::State, routing::{get, post}, Json, Router};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{dto::user_dto::{CreateUser, LoginDto}, entities::user::{self, Column}, error::AppError, state::AppState, utils::{jwt::generate_tokken, password_util::{encrypt_password, verify_password}}};

pub fn auth_router(state:AppState) -> Router{
  Router::new()
  .route("/login", post(login))
  .route("/signup", post(signup))
  .route("/", get(users))
  .with_state(state)
}

async fn login(
  State(state): State<AppState>,
  Json(payload): Json<LoginDto>
) -> Result<String, AppError>{
  let user = user::Entity::find().filter(Column::Email.eq(payload.email))
  .one(&state.db).await?.ok_or_else(|| "User not found").unwrap();
  if !verify_password(&payload.password, &user.password).unwrap(){
    return Err(AppError::Unauthorized("invalid creadential".to_string()));
  }

  Ok(generate_tokken(user.id).unwrap())
}

async fn signup(
  State(state): State<AppState>,
  Json(payload): Json<CreateUser>
) -> Result<Json<user::Model>, AppError>{
  let user = user::ActiveModel{
    name: Set(payload.name),
    phone: Set(payload.phone),
    email: Set(payload.email),
    password: Set(encrypt_password(&payload.password).unwrap()),
    created_at: Set(Utc::now().naive_utc()),
    updated_at: Set(Utc::now().naive_utc()),
    ..Default::default()
  };
  let user = user.insert(&state.db).await?;
  Ok(Json(user))
}

async fn users(
  State(state): State<AppState>,
) -> Result<Json<Vec<user::Model>>, AppError>{
  let users = user::Entity::find().all(&state.db).await?;
  Ok(Json(users))
}
