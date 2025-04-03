use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};

use crate::{error::AppError, state::AppState, utils::jwt::decode_token};

pub async fn auth_midleware(
  State(state): State<AppState>,
  mut req: Request<Body>,
  next: Next
) -> Result<Response, AppError>{
  println!("hi goo");
  let token = req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized("Missing token".to_string()));

  let token_data = decode_token(token.unwrap());
  println!("hello bhai {:?}", &token_data);

  req.extensions_mut().insert(token_data.claims);
  Ok(next.run(req).await)

}