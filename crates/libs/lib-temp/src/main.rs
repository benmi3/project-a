#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    todos: Vec<TodoTemplate>,
}

async fn index(
    axum::extract::State(pool): axum::extract::State<db::DBPool>,
) -> Result<IndexTemplate, (axum::http::StatusCode, String)> {
    sqlx::query_as!(TodoTemplate, "SELECT * FROM todo")
        .fetch_all(&pool)
        .await
        .map(|todos| IndexTemplate { todos })
        .map_err(db::map_db_err)
}
