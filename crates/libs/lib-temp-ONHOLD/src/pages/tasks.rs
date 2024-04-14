use core::fmt;

// Import Task Struct only.
// Actually getting the data, will be through RPC
// The main purpos of the lib-temp is to not only have a
// somewhat usable frontend, but also to make sure the rpc
// routes works as they should.

use crate::config::front_config;
use crate::config::BaseSettings;
use crate::pages::components::task::TaskTemplate;

#[derive(askama::Template)]
#[template(path = "tasks.html")]
struct TasksTemplate {
	base_settings: BaseSettings,
	tasks: Vec<TaskTemplate>,
}

async fn tasks() -> Result<TasksTemplate, (axum::http::StatusCode, String)> {
	let base = BaseSettings {
		page_title: format!("{} Index", front_config().SITE_NAME),
		page_name: "Index".to_string(),
	};
	TasksTemplate {
		base_settings: base,
	}
}

