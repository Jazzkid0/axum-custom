use axum::{routing::get, Router, response::Html};
use askama::Template;
use tower_http::services::ServeDir;
use tokio::fs::read_to_string;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Post {
    pub post_title: String,
    pub post_date: String,
    pub post_body: String,
}

#[tokio::main]
async fn main() {

    // get posts

    let app = Router::new()
        .route("/", get(root))
        .route("/about", get(about))
        .route("/testblog", get(testblog).post(blog2));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> { 
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width-device-width, initial-scale=1">
    <title>homepage</title>
</head>
<body>
    <h1>Lfg</h1>
    <p>Yo waddup</p>

    <ul>
        <li><a href="/about">about</a></li>
        <li><a href="/testblog">blog</a></li>
    </ul>
</body>
"#.to_string())
}

async fn about() -> Html<String> { Html(read_to_string("./assets/about.html").await.unwrap()) }

async fn testblog() -> Html<String> { 
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width-device-width, initial-scale=1">
    <title>rust blog</title>
</head>
<body>
    <h1>Blog woo</h1>
    <p>Idk if html will serve properly like this</p>
    <h2>let's try a list</h2>
    <ul>
        <li>Post number 1</li>
        <li>Post number 2</li>
    </ul>
</body>
</html>
"#.to_string())
}
async fn blog2() -> String { "Yeah maybe someday".to_string() }
