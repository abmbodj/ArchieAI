// Simple runner for ArchieAI Rust modules
// Demonstrates the basic functionality of DataCollector, SessionManager, and AiInterface
// For the record, I hate Rust with a burning passion but here we are
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{fs, net::SocketAddr};

use archie_ai_rust::{AiInterface, DataCollector, SessionManager};
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    // Attempt to read the content of "frontend/index.html"
    match fs::read_to_string("src\\templates\\index.html") {
        Ok(content) => Html(content), // If successful, wrap the content in Html
        Err(e) => Html(format!("<h1>Error reading HTML file: {}</h1>", e)), // Handle errors
    }
}