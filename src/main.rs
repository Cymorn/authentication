mod utils;
use axum::Json;
use axum::{routing::{get, post}, Router};
use serde::Deserialize;
use utils::password::{hash_password, verify_password};
use utils::jwt::create_token;
use uuid::Uuid;

const STORED_HASH: &str =
    "$argon2id$v=19$m=19456,t=2,p=1$test$QWJjZGVmZ2hpamtsbW5vcA";
 #[derive(Deserialize)]
 struct LoginInput {
    password: String,

 }
async fn login(Json(input): axum::Json<LoginInput>) -> axum::Json<String> {
    let is_valid = verify_password(&input.password, STORED_HASH);

    if !is_valid {
        return axum::Json("invalid password".to_string());
    }

    let user_id = Uuid::new_v4();
    let token = create_token(user_id);

    axum::Json(token)
}


async fn root() -> &'static str {
    "Auth server running"
}




#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/login", post(login));


    println!("Server running on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
