// region:    --- Modules

mod config;
mod error;
mod log;
mod web;

pub use self::error::{Error, Result};
use config::web_config;

use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::mw_stamp::mw_req_stamp;
use crate::web::routes_rpc::RpcState;
use crate::web::{routes_login, routes_static};
use axum::{
	http::{HeaderValue, Method},
	middleware, Router,
};
use lib_core::_dev_utils;
use lib_core::model::ModelManager;
use lib_utils::setup::check_setup;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer}; // Needed for preflight
use tracing::info;
use tracing_subscriber::EnvFilter;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time() // For early local development.
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- FOR DEV ONLY
	// TODO
	// Make this more usefull
	if check_setup() {
		_dev_utils::init_dev().await;
	}

	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	// -- Define Routes
	let rpc_state = RpcState { mm: mm.clone() };
	let routes_rpc = web::routes_rpc::routes(rpc_state)
		.route_layer(middleware::from_fn(mw_ctx_require));

	let cors_check = CorsLayer::new()
		//.allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
		.allow_origin(Any) // For Testing
		.allow_headers(Any) // FOr Testing
		.allow_methods([Method::POST, Method::OPTIONS]);

	let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
		.nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_reponse_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(middleware::from_fn(mw_req_stamp))
		.layer(CookieManagerLayer::new())
		.layer(cors_check)
		.fallback_service(routes_static::serve_dir());

	// region:    --- Start Server
	// Note: For this block, ok to unwrap.
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}
