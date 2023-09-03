#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::env;

use anyhow::Result;
use dotenvy::dotenv;
use redis::aio::ConnectionManager;
use tracing_subscriber::{
	prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

mod axum;
mod config;
mod errors;
mod migrations;
mod models;
mod routes;
mod server;
mod shutdown;
mod webhooks;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	tracing_subscriber::registry()
		.with(
			tracing_subscriber::fmt::layer().with_filter(
				EnvFilter::try_from_default_env().unwrap_or_else(|_| "dyson=info".into()),
			),
		)
		.init();

	ensemble::setup(&env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable"))
		.await?;

	let redis_pool = ConnectionManager::new(redis::Client::open(
		env::var("REDIS_URL").expect("Missing REDIS_URL environment variable"),
	)?)
	.await?;

	ensemble::migrate!(
		migrations::CreateUsersTable,
		migrations::CreateTokensTable,
		migrations::CreatePredictionsTable
	)
	.await
	.expect("Failed to run migrations");

	server::start(redis_pool).await
}
