use std::sync::Arc;
use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: usize,
    pub msg: String,
    pub likes: u32,
}

#[derive(Clone, Default)]
pub struct AppState {
    pub posts: Arc<Mutex<Vec<Post>>>,
}
