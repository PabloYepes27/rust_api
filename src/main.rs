use axum::{
    body::Body,
    extract::{Path, Query, Json, Extension},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, delete},
    Router,
    Server
};

use dotenv::dotenv;
use serde_json::json;
use sqlx::{MySqlPool, Row};
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

// Define the get_users function as before
async fn get_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
}

#[tokio::main]
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve the DATABASE_URL environment variable
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable not set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Could not connect to the database");

    // Define Routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(create_user))
        .route("/item/:id", get(show_item))
        .route("/add-item", post(add_item))
        .route("/delete-user/:user_id", delete(delete_user))
        .route("/users", get(get_users))
        .layer(Extension(pool));

    println!("Running on http://localhost:3000");
    // Start Server
    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
