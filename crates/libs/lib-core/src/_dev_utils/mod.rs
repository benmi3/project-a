// region:    --- Modules

mod dev_db;

use crate::ctx::Ctx;
use crate::model::project::{ProjectBmc, ProjectForCreate};
use crate::model::task::{Task, TaskBmc, TaskForCreate};
use crate::model::taskprogress::{
	TaskProgress, TaskProgressBmc, TaskProgressForCreate,
};
use crate::model::tasktime::{TaskTime, TaskTimeBmc, TaskTimeForCreate};
use crate::model::timerecord::{TimeRecord, TimeRecordBmc, TimeRecordForCreate};
use crate::model::{self, ModelManager};
use lib_utils::time::now_utc;
use tokio::sync::OnceCell;
use tracing::info;

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

pub async fn seed_task(
	ctx: &Ctx,
	mm: &ModelManager,
	project_id: i64,
	title: &str,
) -> model::Result<i64> {
	let id = TaskBmc::create(
		ctx,
		mm,
		TaskForCreate {
			project_id,
			title: title.to_string(),
		},
	)
	.await?;
	Ok(id)
}

pub async fn seed_timerecords(
	ctx: &Ctx,
	mm: &ModelManager,
	places: &[&str],
) -> model::Result<Vec<TimeRecord>> {
	let mut timerecords = Vec::new();

	for place in places {
		let id = TimeRecordBmc::create(
			ctx,
			mm,
			TimeRecordForCreate {
				place: place.to_string(),
				start_time: Some(now_utc()),
				stop_time: Some(now_utc()),
			},
		)
		.await?;
		let timerecord = TimeRecordBmc::get(ctx, mm, id).await?;

		timerecords.push(timerecord);
	}

	Ok(timerecords)
}

pub async fn seed_tasktimes(
	ctx: &Ctx,
	mm: &ModelManager,
	task_id: i64,
	comments: &[&str],
) -> model::Result<Vec<TaskTime>> {
	let mut tasktimes = Vec::new();

	for comment in comments {
		let id = TaskTimeBmc::create(
			ctx,
			mm,
			TaskTimeForCreate {
				task_id,
				comment: comment.to_string(),
				start_time: Some(now_utc()),
				stop_time: Some(now_utc()),
			},
		)
		.await?;
		let tasktime = TaskTimeBmc::get(ctx, mm, id).await?;

		tasktimes.push(tasktime);
	}

	Ok(tasktimes)
}

pub async fn seed_taskprogresses(
	ctx: &Ctx,
	mm: &ModelManager,
	task_id: i64,
	progresses: &[i32],
) -> model::Result<Vec<TaskProgress>> {
	let mut taskprogresses = Vec::new();

	for progress in progresses {
		let id = TaskProgressBmc::create(
			ctx,
			mm,
			TaskProgressForCreate {
				task_id,
				progress: *progress,
			},
		)
		.await?;
		let taskprogress = TaskProgressBmc::get(ctx, mm, id).await?;

		taskprogresses.push(taskprogress);
	}

	Ok(taskprogresses)
}
