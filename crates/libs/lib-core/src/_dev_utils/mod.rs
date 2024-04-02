// region:    --- Modules

mod dev_db;

use crate::ctx::Ctx;
use crate::model::project::{ProjectBmc, ProjectForCreate};
use crate::model::task::{Task, TaskBmc, TaskForCreate};
use crate::model::timerecord::{TimeRecord, TimeRecordBmc, TimeRecordForCreate};
use crate::model::{self, ModelManager};
use tokio::sync::OnceCell;
use tracing::info;
use lib_utils::time::now_utc;

// endregion: --- Modules

/// Initialize environment for local development.
/// (for early development, will be called from main()).
pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();

	INIT.get_or_init(|| async {
		info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

		dev_db::init_dev_db().await.unwrap();
	})
	.await;
}

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INIT
		.get_or_init(|| async {
			init_dev().await;
			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}

pub async fn seed_project(
	ctx: &Ctx,
	mm: &ModelManager,
	name: &str,
) -> model::Result<i64> {
	ProjectBmc::create(
		ctx,
		mm,
		ProjectForCreate {
			name: name.to_string(),
		},
	)
	.await
}

pub async fn seed_tasks(
	ctx: &Ctx,
	mm: &ModelManager,
	project_id: i64,
	titles: &[&str],
) -> model::Result<Vec<Task>> {
	let mut tasks = Vec::new();

	for title in titles {
		let id = TaskBmc::create(
			ctx,
			mm,
			TaskForCreate {
				project_id,
				title: title.to_string(),
			},
		)
		.await?;
		let task = TaskBmc::get(ctx, mm, id).await?;

		tasks.push(task);
	}

	Ok(tasks)
}

pub async fn seed_timerecords(
	ctx: &Ctx,
	mm: &ModelManager,
	project_id: i64,
	places: &[&str],
) -> model::Result<Vec<TimeRecord>> {
	let mut timerecords = Vec::new();

	for place in places {
		let id = TimeRecordBmc::create(
			ctx,
			mm,
			TimeRecordForCreate {
				project_id,
				place: place.to_string(),
				start_time: now_utc(),
				stop_time: now_utc(),
			},
		)
			.await?;
		let timerecord = TimeRecordBmc::get(ctx, mm, id).await?;

		timerecords.push(timerecord);
	}

	Ok(timerecords)
}
