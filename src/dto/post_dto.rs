use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreatePost {
    pub title: String,
    pub text: String,
    pub user_id: i32,
}
