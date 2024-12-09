// #![allow(unused)] // For early development.

/* /
:clear
# -- Async
:dep tokio = { version = "1", features = ["full"] }
:dep async-trait = "0.1"
# -- Json
:dep serde = { version = "1", features = ["derive"] }
:dep serde_json = "1"
:dep serde_with = "3"
# -- Web
:dep axum = {version = "0.7", features = ["macros"]}
:dep tower-http = { version = "0.5", features = ["fs"] }
:dep tower-cookies = "0.10"
# -- Tracing
:dep tracing = "0.1"
:dep tracing-subscriber = { version = "0.3", features = ["env-filter"] }
# -- Others
:dep uuid = {version = "1", features = ["v4","fast-rng",]}
:dep strum_macros = "0.25"
:dep derive_more = {version = "1.0.0-beta", features = ["from"] }
# -- App Libs
:dep lib-utils = { path = "./crates/libs/lib-utils"}
:dep lib-rpc = { path = "./crates/libs/lib-rpc"}
:dep lib-auth = { path = "./crates/libs/lib-auth"}
:dep lib-core = { path = "./crates/libs/lib-core"}
// */

// region:    --- Modules

mod config;
mod error;
mod log;
mod web;

pub use self::error::{Error, Result};
use config::web_config;

use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_rpc, routes_static};
use axum::{middleware, Router};
use lib_core::_dev_utils;
use lib_core::model::ModelManager;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
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
	_dev_utils::init_dev().await;

	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	// -- Define Routes
	let routes_rpc = routes_rpc::routes(mm.clone())
		.route_layer(middleware::from_fn(mw_ctx_require));

	let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
		.nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_reponse_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new())
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
