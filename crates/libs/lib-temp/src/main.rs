#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    page_title: String,
}

async fn index() -> Result<IndexTemplate, (axum::http::StatusCode, String)> {
    IndexTemplate
}
