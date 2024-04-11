#[derive(askama::Template)]
#[template(path = "components/task.html")]
pub struct TaskTemplate {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub done: bool,
}
