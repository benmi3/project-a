use crate::router::RpcRouter;
use crate::rpc_router;
use crate::Result;
use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use lib_core::ctx::Ctx;
use lib_core::model::timerecord::{
	TimeRecord, TimeRecordBmc, TimeRecordFilter, TimeRecordForCreate, TimeRecordForUpdate,
};
use lib_core::model::ModelManager;

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		// Same as RpcRouter::new().add...
		create_timerecord,
		list_timerecord,
		update_timerecord,
		delete_timerecord,
	)
}

pub async fn create_timerecord(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<TimeRecordForCreate>,
) -> Result<TimeRecord> {
	let ParamsForCreate { data } = params;

	let id = TimeRecordBmc::create(&ctx, &mm, data).await?;
	let timerecord = TimeRecordBmc::get(&ctx, &mm, id).await?;

	Ok(timerecord)
}

pub async fn list_timerecord(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<TimeRecordFilter>,
) -> Result<Vec<TimeRecord>> {
	let timerecords =
		TimeRecordBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(timerecords)
}

pub async fn update_timerecord(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<TimeRecordForUpdate>,
) -> Result<TimeRecord> {
	let ParamsForUpdate { id, data } = params;

	TimeRecordBmc::update(&ctx, &mm, id, data).await?;

	let timerecord = TimeRecordBmc::get(&ctx, &mm, id).await?;

	Ok(timerecord)
}

pub async fn delete_timerecord(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<TimeRecord> {
	let ParamsIded { id } = params;

	let timerecord = TimeRecordBmc::get(&ctx, &mm, id).await?;
	TimeRecordBmc::delete(&ctx, &mm, id).await?;

	Ok(timerecord)
}
