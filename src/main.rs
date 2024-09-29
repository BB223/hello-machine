use std::{thread, time::Duration};

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use gethostname::gethostname;
use tokio::task;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/api/hello", get(handler)).route("/healthz", get(|| async { (StatusCode::OK, "OK") }));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    // let _ = task::spawn_blocking(|| {
    //     simulate_cpu_work();
    // })
    // .await;
    let message = String::from("Hello, ");
    let message = match gethostname().to_str() {
        Some(h) => message + h + "!",
        None => message + "World!",
    };

    (StatusCode::OK, axum::Json(Message::from(message)))
}

fn simulate_cpu_work() -> () {
    let heavy_iterations = 50_000_000;
    let mut _counter = 0;

    // Simulate CPU-heavy work by doing a large number of iterations
    for i in 0..heavy_iterations {
        _counter += i;
        if i % 10_000_000 == 0 {
            thread::sleep(Duration::from_millis(50)); // Simulate occasional I/O-like waits
        }
    }
}

#[derive(serde::Serialize)]
struct Message {
    pub message: String,
}

impl From<String> for Message {
    fn from(value: String) -> Self {
        Self { message: value }
    }
}
