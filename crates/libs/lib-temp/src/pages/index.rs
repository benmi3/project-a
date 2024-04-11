use crate::config::front_config;


struct BaseSettings {
    page_title: String,
    page_name: String,
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    base_settings: BaseSettings,
}

async fn index() -> Result<IndexTemplate, (axum::http::StatusCode, String)> {
    let base = BaseSettings {
            page_title: format!("{} Index", front_config().SITE_NAME),
            page_name: "Index".to_string(),
        };
    IndexTemplate {
        base_settings: base,
    }
}
