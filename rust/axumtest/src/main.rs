use axum::{response::Html, routing::get, Router};
use axum::{
    http::Method,
    routing::{post, put},
};

use tower_http::{
    cors::{Any, CorsLayer,AllowedOrigins},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
     // build our application with a single route
     let app = Router::new().route("/", get(|| async { "Hello, World!" }))
     .layer(CorsLayer::permissive()).layer(TraceLayer::new_for_http());

     let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000").await.expect("error");
     
     axum::serve(listener, app)
         .await.expect("error");
}

