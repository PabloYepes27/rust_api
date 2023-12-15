use axum::{
    body::Body,
    extract::{Path, Query, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, delete},
    Router,
};

use serde::{Serialize, Deserialize};

// A struct for query parameters
#[derive(Deserialize)]
struct Page {
    number: u32,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// A struct for the JSON body
#[derive(Deserialize)]
struct Item {
    title: String,
}

// Handler for /create-user
async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}
// Handler for /users
async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string(),
        },
        User {
            id: 2,
            name: "John".to_string(),
            email: "john@doe.com".to_string(),
        },
    ];
    Json(users)
}

// A handler to demonstrate path and query extractors
async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number)
}

// A handler to demonstrate the JSON body extractor
async fn add_item(Json(item): Json<Item>) -> String {
    format!("Added item: {}", item.title)
}

// Define a handler that performs an operation and may return an error
async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<User>, impl IntoResponse> {
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(User {
            id: user_id,
            name: "Deleted User".into(),
            email: "some email".to_string(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        )),
    }
}

// Hypothetical async function to delete a user by ID
async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    // Simulate an error for demonstration
    if user_id == 1 {
        Err("User cannot be deleted.".to_string())
    } else {
        // Logic to delete a user...
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Define Routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(create_user))
        .route("/item/:id", get(show_item))
        .route("/add-item", post(add_item))
        .route("/delete-user/:user_id", delete(delete_user))
        .route("/users", get(list_users));

    println!("Running on http://localhost:3000");
    // Start Server
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
