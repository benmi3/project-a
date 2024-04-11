struct BaseSetting {
    page_title: String,
    page_name: String,
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    BaseSettings: BaseSetting,
}

async fn index() -> Result<IndexTemplate, (axum::http::StatusCode, String)> {
    base = BaseSettings {
            page_title: "Index",
            page_name: "Index",
        };
    IndexTemplate {
        BaseSettings: base,
    }
}
