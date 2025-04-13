use axum::{
    routing::post,
    Json, Router,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    }
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct Input {
    foo: String
}

#[derive(Serialize)]
struct Response {
    message: String,
}

async fn handle_post(Json(payload): Json<Input>) -> Json<Response> {
    println!("reached request handling! foo: {}", payload.foo);
    let reply = Response {
        message: format!("Hello, {}!", payload.foo),
    };
    Json(reply)
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:6191".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new().route("/post", post(handle_post)).layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
