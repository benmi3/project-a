use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use modql::filter::{
	FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

// region:    --- TaskTime Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct TaskTime {
	pub id: i64,
	pub task_id: i64,

	pub comment: String,
	#[serde_as(as = "Rfc3339")]
	pub start_time: OffsetDateTime,
	#[serde_as(as = "Rfc3339")]
	pub stop_time: OffsetDateTime,

	// -- Timestamps
	//    (creator and last modified user_id/time)
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[serde_as]
#[derive(Fields, Deserialize)]
pub struct TaskTimeForCreate {
	pub task_id: i64,
	pub comment: String,
	#[serde_as(as = "Option<Rfc3339>")]
	pub start_time: Option<OffsetDateTime>,
	#[serde_as(as = "Option<Rfc3339>")]
	pub stop_time: Option<OffsetDateTime>,
}

#[serde_as]
#[derive(Fields, Deserialize, Default)]
pub struct TaskTimeForUpdate {
	pub comment: Option<String>,
	#[serde_as(as = "Option<Rfc3339>")]
	pub start_time: Option<OffsetDateTime>,
	#[serde_as(as = "Option<Rfc3339>")]
	pub stop_time: Option<OffsetDateTime>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TaskTimeFilter {
	id: Option<OpValsInt64>,
	task_id: Option<OpValsInt64>,
	comment: Option<OpValsString>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	start_time: Option<OpValsValue>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	stop_time: Option<OpValsValue>,

	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}
// endregion: --- TaskTime Types

// region:    --- TaskTimeBmc
pub struct TaskTimeBmc;

impl DbBmc for TaskTimeBmc {
	const TABLE: &'static str = "tasktime";
}

impl TaskTimeBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		tasktime_c: TaskTimeForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, tasktime_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<TaskTime> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<TaskTimeFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<TaskTime>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		tasktime_u: TaskTimeForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, tasktime_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
// endregion: --- TaskTimeBmc

// region: --- TaskTimePreFormat
#[derive(Fields, Deserialize)]
pub struct TaskTimeForCreatePre {
	pub task_id: i64,
	pub comment: String,
	pub start_time: String,
	pub stop_time: String,
}

// endregion: --- TaskTimePreFormat

// region:    --- Tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::_dev_utils;
	use crate::model::project::ProjectBmc;
	use crate::model::Error;
	use anyhow::Result;
	use lib_utils::time::{format_time, now_utc};
	use modql::filter::OpValString;
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
		let fx_comment = "test_create_ok comment";
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"test_create_ok project for time record ",
		)
		.await?;
		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"test_create_ok task for time record ",
		)
		.await?;

		// -- Exec
		let tasktime_c = TaskTimeForCreate {
			task_id: fx_task_id,
			comment: fx_comment.to_string(),
			start_time: Some(now_utc()),
			stop_time: Some(now_utc()),
		};
		let id = TaskTimeBmc::create(&ctx, &mm, tasktime_c).await?;

		// -- Check
		let tasktime = TaskTimeBmc::get(&ctx, &mm, id).await?;
		assert_eq!(tasktime.comment, fx_comment);

		// -- Clean
		TaskTimeBmc::delete(&ctx, &mm, id).await?;

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
		let res = TaskTimeBmc::get(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound {
					entity: "tasktime",
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
		let fx_titles = &[
			"test_list_all_ok-tasktime 01",
			"test_list_all_ok-tasktime 02",
		];
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"test_list_all_ok project for tasktime",
		)
		.await?;
		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"test_create_ok task for tasktime ",
		)
		.await?;

		_dev_utils::seed_tasktimes(&ctx, &mm, fx_task_id, fx_titles).await?;

		// -- Exec
		let filter = TaskTimeFilter {
			task_id: Some(fx_task_id.into()),
			..Default::default()
		};
		let tasktimes =
			TaskTimeBmc::list(&ctx, &mm, Some(vec![filter]), None).await?;

		// -- Check
		assert_eq!(tasktimes.len(), 2, "number of seeded tasktimes.");

		// -- Clean
		ProjectBmc::delete(&ctx, &mm, fx_project_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_by_comment_contains_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_comments = &[
			"test_list_by_comment_contains_ok comment01",
			"test_list_by_comment_contains_ok comment02.1",
			"test_list_by_comment_contains_ok comment02.2",
		];
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"test_list_by_title_contains_ok project for tasktimes ",
		)
		.await?;
		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"test_list_by_progress_contains_ok task for tasktimes",
		)
		.await?;

		_dev_utils::seed_tasktimes(&ctx, &mm, fx_task_id, fx_comments).await?;

		// -- Exec
		let filter = TaskTimeFilter {
			task_id: Some(fx_task_id.into()),
			comment: Some(
				OpValString::Contains(
					"by_comment_contains_ok comment02".to_string(),
				)
				.into(),
			),
			..Default::default()
		};
		let tasktimes =
			TaskTimeBmc::list(&ctx, &mm, Some(vec![filter]), None).await?;

		// -- Check
		assert_eq!(tasktimes.len(), 2);

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
		let fx_comments = &[
			"test_list_with_list_options_ok comment1",
			"test_list_with_list_options_ok comment2.0",
			"test_list_with_list_options_ok comment2.1",
		];
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"test_list_with_list_options_ok project for tasktime ",
		)
		.await?;

		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"test_list_with_list_options_ok task for tasktime",
		)
		.await?;

		_dev_utils::seed_tasktimes(&ctx, &mm, fx_task_id, fx_comments).await?;

		// -- Exec
		let filter: TaskTimeFilter = TaskTimeFilter {
			task_id: Some(fx_task_id.into()),
			..Default::default()
		};
		let list_options: ListOptions = serde_json::from_value(json! ({
			"offset": 0,
			"limit": 2,
			"order_bys": "!comment"
		}))?;
		let tasktimes =
			TaskTimeBmc::list(&ctx, &mm, Some(vec![filter]), Some(list_options))
				.await?;

		// -- Check
		let comments: Vec<String> =
			tasktimes.iter().map(|t| t.comment.to_string()).collect();
		assert_eq!(comments.len(), 2);
		assert_eq!(
			&comments,
			&[
				"test_list_with_list_options_ok comment2.1",
				"test_list_with_list_options_ok comment2.0"
			]
		);

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
		let fx_comment = "test_update_ok - comment 01";
		let fx_comment_new = "test_update_ok - comment 01 - new";
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"test_update_ok project for tasktime",
		)
		.await?;
		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"test_update_ok task for tasktime",
		)
		.await?;

		let fx_tasktime =
			_dev_utils::seed_tasktimes(&ctx, &mm, fx_task_id, &[fx_comment])
				.await?
				.remove(0);

		// -- Exec
		TaskTimeBmc::update(
			&ctx,
			&mm,
			fx_tasktime.id,
			TaskTimeForUpdate {
				comment: Some(fx_comment_new.to_string()),
				..Default::default()
			},
		)
		.await?;

		// -- Check
		let tasktime = TaskTimeBmc::get(&ctx, &mm, fx_tasktime.id).await?;
		assert_eq!(tasktime.comment, fx_comment_new);

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
			"project for tasktime test_list_by_ctime_ok",
		)
		.await?;

		let fx_task_id = _dev_utils::seed_task(
			&ctx,
			&mm,
			fx_project_id,
			"task for tasktime test_list_by_ctime_ok",
		)
		.await?;

		let fx_comments_01 = &[
			"test_list_by_ctime_ok 01.1",
			"test_list_by_ctime_ok 01.2",
			"test_list_by_ctime_ok 01.3",
		];
		_dev_utils::seed_tasktimes(&ctx, &mm, fx_task_id, fx_comments_01).await?;

		let time_marker = format_time(now_utc());
		sleep(Duration::from_millis(300)).await;
		let fx_comments_02 =
			&["test_list_by_ctime_ok 02.1", "test_list_by_ctime_ok 02.2"];
		_dev_utils::seed_tasktimes(&ctx, &mm, fx_task_id, fx_comments_02).await?;

		// -- Exec
		let filter_json = json! ({
			"ctime": {"$gt": time_marker}, // time in Rfc3339
		});
		let filter = vec![serde_json::from_value(filter_json)?];
		let tasktimes = TaskTimeBmc::list(&ctx, &mm, Some(filter), None).await?;

		// -- Check
		let comments: Vec<String> =
			tasktimes.into_iter().map(|t| t.comment).collect();
		assert_eq!(comments.len(), 2);
		assert_eq!(&comments, fx_comments_02);

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
		let res = TaskTimeBmc::delete(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound {
					entity: "tasktime",
					id: 100
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests
