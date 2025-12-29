use axum::{routing::get, Router};
use utils::password::{hash_password, verify_password};
use serde::Deserialize;
use axum::Json;
use axum::{routing::{get, post}, Router};
const STORED_HASH: &str =
    "$argon2id$v=19$m=19456,t=2,p=1$test$QWJjZGVmZ2hpamtsbW5vcA";
 #[derive(Deserialize)]
 struct LoginInput {
    password: String,

 }

 async fn login(Json(input): Json<LoginInput>) -> Json<bool> {
    let is_valid = verify_password(&input.password, STORED_HASH);
    Json(is_valid)
}



#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(|| async { "Hello, backend!" }))
    .route("/register", post(register))
    .route("/login", post(login));

    println!("Server running on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
