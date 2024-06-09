use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use modql::filter::{
	FilterNodes, ListOptions, OpValsInt32, OpValsInt64, OpValsValue,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

// region:    --- Task Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct TaskProgress {
	pub id: i64,
	pub task_id: i64,

	pub progress: i32,

	// -- Timestamps
	//    (creator and last modified user_id/time)
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct TaskProgressForCreate {
	pub progress: i32,
	pub task_id: i64,
}

#[derive(Fields, Deserialize, Default)]
pub struct TaskProgressForUpdate {
	pub progress: i32,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TaskProgressFilter {
	id: Option<OpValsInt64>,
	task_id: Option<OpValsInt64>,
	progress: Option<OpValsInt32>,

	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}
// endregion: --- Task Types

// region:    --- TaskBmc
pub struct TaskProgressBmc;

impl DbBmc for TaskProgressBmc {
	const TABLE: &'static str = "taskprogress";
}

impl TaskProgressBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		taskprogress_c: TaskProgressForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, taskprogress_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<TaskProgress> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<TaskProgressFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<TaskProgress>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		taskprogress_u: TaskProgressForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, taskprogress_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
// endregion: --- TaskBmc

// region:    --- Tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::_dev_utils;
	use crate::model::project::ProjectBmc;
	use crate::model::Error;
	use anyhow::Result;
	use lib_utils::time::{format_time, now_utc};
	use serde_json::json;
	use serial_test::serial;
	use std::time::Duration;
	use tokio::time::sleep;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_progress = 45;
		let fx_project_id =
			_dev_utils::seed_project(&ctx, &mm, "test_create_ok project for task ")
				.await?;
		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"test_create_ok task for taskprogress",
		)
		.await?;

		// -- Exec
		let taskprogress_c = TaskProgressForCreate {
			task_id: fx_task_id,
			progress: fx_progress,
		};
		let id = TaskProgressBmc::create(&ctx, &mm, taskprogress_c).await?;

		// -- Check
		let taskprogress = TaskProgressBmc::get(&ctx, &mm, id).await?;
		assert_eq!(taskprogress.progress, fx_progress);

		// -- Clean
		TaskProgressBmc::delete(&ctx, &mm, id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = TaskProgressBmc::get(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound {
					entity: "taskprogress",
					id: 100
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_all_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_taskprogress_progress = &[15, 55, 80];
		let fx_project_id =
			_dev_utils::seed_project(&ctx, &mm, "test_list_all_ok project for task")
				.await?;
		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"test_list_all_ok task for taskprogress",
		)
		.await?;
		_dev_utils::seed_taskprogresses(
			&ctx,
			&mm,
			fx_task_id,
			fx_taskprogress_progress,
		)
		.await?;

		// -- Exec
		let filter = TaskProgressFilter {
			task_id: Some(fx_task_id.into()),
			..Default::default()
		};
		let taskprogresses =
			TaskProgressBmc::list(&ctx, &mm, Some(vec![filter]), None).await?;

		// -- Check
		assert_eq!(taskprogresses.len(), 3, "number of seeded taskprogresses.");

		// -- Clean
		ProjectBmc::delete(&ctx, &mm, fx_project_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_by_title_contains_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"test_list_by_title_contains_ok project for task ",
		)
		.await?;
		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"test_list_all_ok task for taskprogress",
		)
		.await?;

		_dev_utils::seed_taskprogresses(&ctx, &mm, fx_task_id, &[3, 50, 60]).await?;

		// -- Exec
		let filter = TaskProgressFilter {
			task_id: Some(fx_task_id.into()),
			..Default::default()
		};
		let taskprogresses =
			TaskProgressBmc::list(&ctx, &mm, Some(vec![filter]), None).await?;

		// -- Check
		assert_eq!(taskprogresses.len(), 3);

		// -- Cleanup
		ProjectBmc::delete(&ctx, &mm, fx_project_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_with_list_options_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_titles = &[
			"test_list_with_list_options_ok 01",
			"test_list_with_list_options_ok 02.1",
			"test_list_with_list_options_ok 02.2",
		];
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"test_list_with_list_options_ok project for task ",
		)
		.await?;
		_dev_utils::seed_tasks(&ctx, &mm, fx_project_id, fx_titles).await?;
		let fx_tasks =
			_dev_utils::seed_tasks(&ctx, &mm, fx_project_id, fx_titles).await?;

		// -- Iterate
		for fx_task in fx_tasks {
			// -- Exec

			_dev_utils::seed_taskprogresses(&ctx, &mm, fx_task.id, &[10, 50, 90])
				.await?;

			let filter: TaskProgressFilter = TaskProgressFilter {
				task_id: Some(OpValsInt64::into(fx_task.id.into())),
				..Default::default()
			};
			let list_options: ListOptions = serde_json::from_value(json! ({
				"offset": 0,
				"limit": 2,
				"order_bys": "!progress"
			}))?;
			let taskprogresses = TaskProgressBmc::list(
				&ctx,
				&mm,
				Some(vec![filter]),
				Some(list_options),
			)
			.await?;

			// -- Check
			let progresses: Vec<i32> =
				taskprogresses.iter().map(|t| t.progress).collect();
			assert_eq!(progresses.len(), 2);
			assert_eq!(&progresses, &[90, 50]);
		}

		// -- Cleanup
		// Will delete associated tasks
		ProjectBmc::delete(&ctx, &mm, fx_project_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_update_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_progress = 15;
		let fx_progress_new = 16;
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"test_update_ok project for taskprogress",
		)
		.await?;
		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"test_update_ok - task 01 for taskprogress",
		)
		.await?;

		let fx_taskprogress =
			_dev_utils::seed_taskprogresses(&ctx, &mm, fx_task_id, &[fx_progress])
				.await?
				.remove(0);

		// -- Exec
		TaskProgressBmc::update(
			&ctx,
			&mm,
			fx_taskprogress.id,
			TaskProgressForUpdate {
				progress: fx_progress_new,
				..Default::default()
			},
		)
		.await?;

		// -- Check
		let taskprogress =
			TaskProgressBmc::get(&ctx, &mm, fx_taskprogress.id).await?;

		assert_eq!(taskprogress.progress, fx_progress_new);

		// -- Clean
		ProjectBmc::delete(&ctx, &mm, fx_project_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_by_ctime_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"project for tasks test_list_by_ctime_ok",
		)
		.await?;
		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"test_list_by_ctime_ok 01.1",
		)
		.await?;
		let fx_progress_01 = &[23];
		let fx_progress_02 = &[33, 45];

		_dev_utils::seed_taskprogresses(&ctx, &mm, fx_task_id, fx_progress_01)
			.await?;

		let time_marker = format_time(now_utc());
		sleep(Duration::from_millis(300)).await;

		_dev_utils::seed_taskprogresses(&ctx, &mm, fx_task_id, fx_progress_02)
			.await?;

		// -- Exec
		let filter_json = json! ({
			"ctime": {"$gt": time_marker}, // time in Rfc3339
		});
		let filter = vec![serde_json::from_value(filter_json)?];
		let taskprogresses =
			TaskProgressBmc::list(&ctx, &mm, Some(filter), None).await?;

		// -- Check
		let progresses: Vec<i32> =
			taskprogresses.into_iter().map(|t| t.progress).collect();
		assert_eq!(progresses.len(), 2);
		assert_eq!(&progresses, fx_progress_02);

		// -- Cleanup
		ProjectBmc::delete(&ctx, &mm, fx_project_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_delete_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = TaskProgressBmc::delete(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound {
					entity: "taskprogress",
					id: 100
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests
