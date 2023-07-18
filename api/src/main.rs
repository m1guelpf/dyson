#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use anyhow::Result;
use dotenvy::dotenv;
use tracing_subscriber::{
	prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

#[allow(warnings, unused)]
mod db;

mod routes;
mod server;
mod shutdown;

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

	let prisma_client = db::new_client()
		.await
		.expect("Failed to connect to database");

	#[cfg(debug_assertions)]
	prisma_client._db_push().await?;
	#[cfg(not(debug_assertions))]
	prisma_client._migrate_deploy().await?;

	server::start(prisma_client).await
}
