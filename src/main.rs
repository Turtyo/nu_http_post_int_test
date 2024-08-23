use axum::{routing::post, Json, Router};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/i-like-int", post(what_is_that_int));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn what_is_that_int(Json(value): Json<u64>) -> String {
    println!("We got a value: {}", value);
    "Finished".to_string()
}
