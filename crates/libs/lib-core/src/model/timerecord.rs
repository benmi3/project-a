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

// region:    --- TimeRecord Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct TimeRecord {
	pub id: i64,
	pub task_id: i64,

	pub place: String,
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

#[derive(Fields, Deserialize)]
pub struct TimeRecordForCreate {
	pub task_id: i64,
	pub place: String,
	pub start_time: OffsetDateTime,
	pub stop_time: OffsetDateTime,
}

#[derive(Fields, Deserialize, Default)]
pub struct TimeRecordForUpdate {
	pub place: Option<String>,
	pub start_time: Option<OffsetDateTime>,
	pub stop_time: Option<OffsetDateTime>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TimeRecordFilter {
	id: Option<OpValsInt64>,
	task_id: Option<OpValsInt64>,
	place: Option<OpValsString>,
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
// endregion: --- TimeRecord Types

// region:    --- TimeRecordBmc
pub struct TimeRecordBmc;

impl DbBmc for TimeRecordBmc {
	const TABLE: &'static str = "timerecord";
}

impl TimeRecordBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		timerecord_c: TimeRecordForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, timerecord_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<TimeRecord> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<TimeRecordFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<TimeRecord>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		timerecord_u: TimeRecordForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, timerecord_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
// endregion: --- TimeRecordBmc

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
		let fx_place = "test_create_ok place";
		let fx_project_id =
			_dev_utils::seed_project(&ctx, &mm, "test_create_ok project for time record ")
				.await?;
		let fx_task_id = 
		        _dev_utils::seed_task(&ctx, &mm, fx_project_id, "test_create_ok task for time record ")
				.await?;

		// -- Exec
		let timerecord_c = TimeRecordForCreate {
			task_id: fx_task_id,
			place: fx_place.to_string(),
			start_time: now_utc(),
			stop_time: now_utc(),
		};
		let id = TimeRecordBmc::create(&ctx, &mm, timerecord_c).await?;

		// -- Check
		let timerecord = TimeRecordBmc::get(&ctx, &mm, id).await?;
		assert_eq!(timerecord.place, fx_place);

		// -- Clean
		TimeRecordBmc::delete(&ctx, &mm, id).await?;

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
		let res = TimeRecordBmc::get(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound {
					entity: "timerecord",
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
		let fx_titles = &["test_list_all_ok-timerecord 01", "test_list_all_ok-timerecord 02"];
		let fx_project_id =
			_dev_utils::seed_project(&ctx, &mm, "test_list_all_ok project for timerecord")
				.await?;
		let fx_task_id =
			_dev_utils::seed_task(&ctx, &mm, fx_project_id, "test_create_ok task for time record ")
				.await?;

		_dev_utils::seed_timerecords(&ctx, &mm, fx_task_id, fx_titles).await?;

		// -- Exec
		let filter = TimeRecordFilter {
			task_id: Some(fx_task_id.into()),
			..Default::default()
		};
		let timerecords = TimeRecordBmc::list(&ctx, &mm, Some(vec![filter]), None).await?;

		// -- Check
		assert_eq!(timerecords.len(), 2, "number of seeded timerecords.");

		// -- Clean
		ProjectBmc::delete(&ctx, &mm, fx_project_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_by_place_contains_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_places = &[
			"test_list_by_place_contains_ok place01",
			"test_list_by_place_contains_ok place02.1",
			"test_list_by_place_contains_ok place02.2",
		];
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"test_list_by_title_contains_ok project for timerecords ",
		)
		.await?;
		let fx_task_id = 
			_dev_utils::seed_task(&ctx, &mm, fx_project_id, "test_list_by_progress_contains_ok task for timerecords")
				.await?;

		_dev_utils::seed_timerecords(&ctx, &mm, fx_task_id, fx_places).await?;

		// -- Exec
		let filter = TimeRecordFilter {
			task_id: Some(fx_task_id.into()),
			place: Some(
				OpValString::Contains("by_place_contains_ok place02".to_string()).into(),
			),
			..Default::default()
		};
		let timerecords = TimeRecordBmc::list(&ctx, &mm, Some(vec![filter]), None).await?;

		// -- Check
		assert_eq!(timerecords.len(), 2);

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
		let fx_places = &[
			"test_list_with_list_options_ok place1",
			"test_list_with_list_options_ok place2.0",
			"test_list_with_list_options_ok place2.1",
		];
		let fx_project_id = _dev_utils::seed_project(
			&ctx,
			&mm,
			"test_list_with_list_options_ok project for timerecord ",
		)
		.await?;

		let fx_task_id = 
			_dev_utils::seed_task(&ctx, &mm, fx_project_id, "test_list_with_list_options_ok task for timerecord")
				.await?;

		_dev_utils::seed_timerecords(&ctx, &mm, fx_task_id, fx_places).await?;

		// -- Exec
		let filter: TimeRecordFilter = TimeRecordFilter {
			task_id: Some(fx_task_id.into()),
			..Default::default()
		};
		let list_options: ListOptions = serde_json::from_value(json! ({
			"offset": 0,
			"limit": 2,
			"order_bys": "!place"
		}))?;
		let timerecords =
			TimeRecordBmc::list(&ctx, &mm, Some(vec![filter]), Some(list_options)).await?;

		// -- Check
		let places: Vec<String> =
			timerecords.iter().map(|t| t.place.to_string()).collect();
		assert_eq!(places.len(), 2);
		assert_eq!(
			&places,
			&[
				"test_list_with_list_options_ok place2.1",
				"test_list_with_list_options_ok place2.0"
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
		let fx_place = "test_update_ok - place 01";
		let fx_place_new = "test_update_ok - place 01 - new";
		let fx_project_id =
			_dev_utils::seed_project(&ctx, &mm, "test_update_ok project for timerecord")
				.await?;
		let fx_task_id =
			_dev_utils::seed_task(&ctx, &mm, fx_project_id, "test_update_ok task for timerecord")
				.await?;

		let fx_timerecord = _dev_utils::seed_timerecords(&ctx, &mm, fx_task_id, &[fx_place])
			.await?
			.remove(0);

		// -- Exec
		TimeRecordBmc::update(
			&ctx,
			&mm,
			fx_timerecord.id,
			TimeRecordForUpdate {
				place: Some(fx_place_new.to_string()),
				..Default::default()
			},
		)
		.await?;

		// -- Check
		let timerecord = TimeRecordBmc::get(&ctx, &mm, fx_timerecord.id).await?;
		assert_eq!(timerecord.place, fx_place_new);

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
			"project for timerecord test_list_by_ctime_ok",
		)
		.await?;

		let fx_task_id =
			_dev_utils::seed_task(&ctx, &mm, fx_project_id, "task for timerecord test_list_by_ctime_ok")
				.await?;

		let fx_places_01 = &[
			"test_list_by_ctime_ok 01.1",
			"test_list_by_ctime_ok 01.2",
			"test_list_by_ctime_ok 01.3",
		];
		_dev_utils::seed_timerecords(&ctx, &mm, fx_task_id, fx_places_01).await?;

		let time_marker = format_time(now_utc());
		sleep(Duration::from_millis(300)).await;
		let fx_places_02 =
			&["test_list_by_ctime_ok 02.1", "test_list_by_ctime_ok 02.2"];
		_dev_utils::seed_timerecords(&ctx, &mm, fx_task_id, fx_places_02).await?;

		// -- Exec
		let filter_json = json! ({
			"ctime": {"$gt": time_marker}, // time in Rfc3339
		});
		let filter = vec![serde_json::from_value(filter_json)?];
		let timerecords = TimeRecordBmc::list(&ctx, &mm, Some(filter), None).await?;

		// -- Check
		let places: Vec<String> = timerecords.into_iter().map(|t| t.place).collect();
		assert_eq!(places.len(), 2);
		assert_eq!(&places, fx_places_02);

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
		let res = TimeRecordBmc::delete(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound {
					entity: "timerecord",
					id: 100
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests
