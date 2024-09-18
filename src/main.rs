use axum::{routing::get, Router};
use gethostname::gethostname;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/hello", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> String {
    let message = String::from("Hello, ");
    let message = match gethostname().to_str() {
        Some(h) => message + h,
        None => message,
    } + "\n";
    message
}
