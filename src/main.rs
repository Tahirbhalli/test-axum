mod db;
mod dto;
mod handlers;
mod midlewares;
mod routes;
use dotenv::dotenv;
use routes::create_routes;
use state::AppState;
use std::env;
mod entities;
mod error;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = db::establish_connection(&database_url).await.unwrap();
    println!("{database_url}");
    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("server runngin");
    let app_state: AppState = AppState { db };
    axum::serve(listner, create_routes(app_state))
        .await
        .unwrap();
}
