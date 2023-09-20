use std::time::Duration;

use clap::{ArgAction, Parser};
use tokio::sync::mpsc;
use tracing_subscriber::{prelude::*, EnvFilter};
use url::Url;

use crate::{director::Director, health_check::HealthChecker, shutdown::Shutdown};

mod director;
mod health_check;
mod redis;
mod server;
mod shutdown;
mod state;

#[derive(Debug, Parser)]
struct Args {
	/// The URL of the Redis instance to connect to.
	#[clap(long, required = true, action = ArgAction::Append)]
	redis_url: Vec<Url>,
	/// The URL of the Cog HTTP base.
	#[clap(long, default_value = "http://localhost:5000")]
	cog_http_base: Url,
	/// The names of the Redis queues to read from.
	#[clap(long, required = true, action = ArgAction::Append)]
	redis_input_queues: Vec<String>,
	/// The names of the Redis keys to read.
	#[clap(long, required = true, action = ArgAction::Append)]
	redis_consumer_ids: Vec<String>,
	/// The time in seconds to wait for a prediction to be made.
	#[clap(long, default_value = "1800")]
	predict_timeout: u64,
	/// Maximum number of consecutive failures before the worker should exit
	#[clap(long, default_value = "5")]
	max_consecutive_failures: u64,
	/// Webhook URL to report instance state to.
	#[clap(long)]
	report_instance_state_url: Option<Url>,
	/// Webhook URL to report setup run state to.
	#[clap(long)]
	report_setup_run_url: Option<Url>,
	/// Timeout (in seconds) before director aborts waiting for model container setup. `0` == no timeout
	#[clap(long, default_value = "0")]
	model_setup_timeout: u64,
	/// Whether Director should run in hotswap mode, enabling it to swap between weights and queues as needed
	#[clap(long)]
	hotswap_mode: bool,
}

#[tokio::main]
async fn main() {
	tracing_subscriber::registry()
		.with(
			tracing_subscriber::fmt::layer().with_filter(
				EnvFilter::try_from_default_env().unwrap_or_else(|_| "dyson=info".into()),
			),
		)
		.init();

	let args = Args::parse();
	assert!(
		args.redis_url.len() == args.redis_input_queues.len() && args.redis_url.len() == args.redis_consumer_ids.len(),
		"--redis-url, --redis-input-queues, and --redis-consumer-ids must have the same number of elements"
	);
	let redis_args = args
		.redis_url
		.into_iter()
		.zip(args.redis_input_queues)
		.zip(args.redis_consumer_ids)
		.map(|((url, queue), consumer_id)| (url, queue, consumer_id))
		.collect::<Vec<_>>();

	let shutdown = Shutdown::new().unwrap();
	let (tx, rx) = mpsc::channel(128);

	let state = state::build(tx.clone());
	let mut healthcheck = HealthChecker::new(tx, args.cog_http_base.clone());

	let mut director = Director::new(
		rx,
		healthcheck.agent(),
		Duration::from_secs(args.predict_timeout),
		args.max_consecutive_failures,
		args.report_setup_run_url,
		args.cog_http_base,
		Duration::from_secs(args.model_setup_timeout),
	);

	tokio::select! {
		_ = director.start() => {},
		_ = shutdown.handle() => {},
		_ = healthcheck.run() => {},
		_ = server::start(shutdown.clone(), state) => {},
	}
}
