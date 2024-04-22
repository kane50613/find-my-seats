pub mod error;
pub mod vieshow;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Movie {
    id: String,
    title: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    movies: Vec<Movie>,
}

async fn index() -> impl IntoResponse {
    let movies = vieshow::list_movies().await.unwrap();

    IndexTemplate { movies }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(index));

    Ok(router.into())
}
