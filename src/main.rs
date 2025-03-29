mod handlers;
mod midlewares;
mod routes;
mod state;

use routes::create_routes;

#[tokio::main]
async fn main() {
    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("server runngin");
    axum::serve(listner, create_routes()).await.unwrap();
}
