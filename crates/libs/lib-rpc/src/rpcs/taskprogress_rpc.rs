use crate::router::RpcRouter;
use crate::rpc_router;
use crate::Result;
use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use lib_core::ctx::Ctx;
use lib_core::model::taskprogress::{
	TaskProgress, TaskProgressBmc, TaskProgressFilter, TaskProgressForCreate, TaskProgressForUpdate,
};
use lib_core::model::ModelManager;

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		// Same as RpcRouter::new().add...
		create_taskprogress,
		list_taskprogresses,
		update_taskprogress,
		delete_taskprogress,
	)
}

pub async fn create_taskprogress(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<TaskProgressForCreate>,
) -> Result<TaskProgress> {
	let ParamsForCreate { data } = params;

	let id = TaskProgressBmc::create(&ctx, &mm, data).await?;
	let task = TaskProgressBmc::get(&ctx, &mm, id).await?;

	Ok(task)
}

pub async fn list_taskprogresses(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<TaskProgressFilter>,
) -> Result<Vec<TaskProgress>> {
	let tasks =
		TaskProgressBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(tasks)
}

pub async fn update_taskprogress(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<TaskProgressForUpdate>,
) -> Result<TaskProgress> {
	let ParamsForUpdate { id, data } = params;

	TaskProgressBmc::update(&ctx, &mm, id, data).await?;

	let task = TaskProgressBmc::get(&ctx, &mm, id).await?;

	Ok(task)
}

pub async fn delete_taskprogress(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<TaskProgress> {
	let ParamsIded { id } = params;

	let task = TaskProgressBmc::get(&ctx, &mm, id).await?;
	TaskProgressBmc::delete(&ctx, &mm, id).await?;

	Ok(task)
}
