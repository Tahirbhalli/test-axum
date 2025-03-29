use axum::{body::Body, http::Request, middleware::Next, response::Response};

pub async fn before_action(req: Request<Body>, next: Next) -> Response {
    println!("it is hitting in before action in {:?}", req.uri());
    next.run(req).await
}

pub async fn after_action(req: Request<Body>, next: Next) -> Response {
    let response = next.run(req).await;

    println!("this behaves like Rails after action");

    response
}

pub async fn round_action(req: Request<Body>, next: Next) -> Response {
    println!("this behaves like Rails round action");

    let response = next.run(req).await;

    println!("this behaves like Rails round action");

    response
}
