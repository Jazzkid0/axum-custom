use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/about", get(about))
        .route("/blog", get(blog).post(blog2));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String { "Yo waddup".to_string() }
async fn about() -> String { "I'm super cool".to_string() }
async fn blog() -> String { "Will I write a blog?\n".to_string() }
async fn blog2() -> String { "Yeah maybe someday".to_string() }
