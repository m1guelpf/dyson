use std::{
	env,
	time::{Duration, Instant},
};

use anyhow::bail;
use chrono::Utc;
use cog_core::http::Status;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde_json::json;
use tokio::{sync::mpsc, time::sleep};
use url::Url;

use crate::{
	health_check::{self, Health},
	state::Event,
};

/// How often to check for cancelation or shutdown signals while a prediction is running.
/// 100ms mirrors the value currently supplied to the `poll` keyword argument for Worker.predict(...) in the redis queue worker code.
const POLL_INTERVAL: Duration = Duration::from_millis(100);

/// How long it's acceptable to wait for a prediction create request to respond.
const PREDICTION_CREATE_TIMEOUT: Duration = Duration::from_secs(5);

/// How long to wait for a cancelation to complete. This is currently very generous to give time for file uploads to complete.
const CANCEL_WAIT: Duration = Duration::from_secs(30);

/// How long to wait for an explicit healthcheck request before aborting. Note
/// that to avoid failing prematurely this should be at least as long as the it
/// takes for the complete chain of retries configured for the Healthchecker.
const HEALTHCHECK_WAIT: Duration = Duration::from_secs(10);

pub struct Director {
	should_exit: bool,
	cog_http_base: Url,
	max_failure_count: u64,
	predict_timeout: Duration,
	model_setup_timeout: Duration,
	cog_client: ClientWithMiddleware,
	health_check: health_check::Agent,
	report_setup_run_url: Option<Url>,
	event_queue: mpsc::Receiver<Event>,
}

impl Director {
	pub fn new(
		event_queue: mpsc::Receiver<Event>,
		health_check: health_check::Agent,
		predict_timeout: Duration,
		max_failure_count: u64,
		report_setup_run_url: Option<Url>,
		cog_http_base: Url,
		model_setup_timeout: Duration,
	) -> Self {
		let cog_client = ClientBuilder::new(reqwest::Client::new())
			.with(RetryTransientMiddleware::new_with_policy(
				ExponentialBackoff::builder().build_with_max_retries(6),
			))
			.build();

		Self {
			cog_client,
			event_queue,
			cog_http_base,
			health_check,
			predict_timeout,
			max_failure_count,
			should_exit: false,
			model_setup_timeout,
			report_setup_run_url,
		}
	}

	pub async fn start(&mut self) {
		if env::var("KUBERNETES_SERVICE_HOST").is_ok() {
			signal_pod_readiness().await;
		}

		// First, we wait for the model container to report a successful setup.
		self.setup().await;

		// Now, we enter the main loop, pulling prediction requests from Redis and managing the model container.
		self.r#loop().await;

		todo!("...");
	}

	async fn setup(&mut self) {
		let instant = Instant::now();

		while !self.should_exit {
			let event = tokio::select! {
				_ = sleep(POLL_INTERVAL) => {
					let elapsed = instant.elapsed();
					let _ = self.abort_if_setup_has_timed_out(elapsed).await;
					tracing::debug!(elapsed_secs = elapsed.as_secs(), "setup: waiting for model container");

					continue;
				},
				event = self.event_queue.recv() => {
					let Some(event) = event else {
						let _ = self.abort("channel closed");
						break;
					};

					event
				},
			};

			let Event::Healthcheck(check) = event else {
				tracing::warn!(event = ?event, "setup: received unexpected event");
				continue;
			};

			if !matches!(check.status, Health::Ready | Health::SetupFailed) {
				tracing::warn!(health = ?check.status, "setup: health status changed");
				continue;
			}

			let elapsed = instant.elapsed();
			tracing::info!(elapsed = ?elapsed, health = ?check.status, "setup: model container finished setup");
			self._report_setup_run(check.setup.unwrap_or_default())
				.await;

			if matches!(check.status, Health::SetupFailed) {
				let _ = self.abort("model container failed setup");
				break;
			}

			self.health_check
				.set_interval(Duration::from_secs(5))
				.await
				.unwrap();
		}
	}

	async fn r#loop(&mut self) {
		let sequence_number = 0;

		while !self.should_exit {
			self.redis_consumer_rotator.rotate();
			let redis_consumer = self.redis_consumer_rotator.get_current();

			let _ = self.confirm_model_health().await;

			todo!("...");
		}
	}

	async fn confirm_model_health(&mut self) -> anyhow::Result<()> {
		self.health_check.request_status().await.unwrap();
		let mark = Instant::now();

		while mark.elapsed() < HEALTHCHECK_WAIT {
			tokio::select! {
				_ = sleep(POLL_INTERVAL) => {
					continue;
				},
				event = self.event_queue.recv() => {
					let Some(event) = event else {
						self.abort("channel closed")?;
						unreachable!("abort() always errors");
					};

					match event {
						Event::Healthcheck(check) => {
							match check.status {
								Health::Ready => {
									return Ok(())
								},
								Health::Busy => {
									continue;
								},
								_ => {
									self.abort(&format!("healthcheck confirmation: model container is not healthy: {:?}", check.status))?;
								}
							}
						},
						_ => {
							tracing::warn!(event = ?event, "healthcheck confirmation: received unexpected event while waiting");
						},
					}
				},
			}
		}

		self.abort("healthcheck confirmation: waited too long without response")?;
		unreachable!("abort() always errors");
	}

	/// Check if the model setup timeout has been hit and abort if timeout is exceeded.
	async fn abort_if_setup_has_timed_out(&mut self, elapsed: Duration) -> anyhow::Result<()> {
		if self.model_setup_timeout.is_zero() || elapsed < self.model_setup_timeout {
			return Ok(());
		}

		let completed_at = Utc::now();
		let started_at = completed_at - elapsed;
		let error_msg = format!(
			"model container failed to boot and complete setup within {} seconds",
			self.model_setup_timeout.as_secs()
		);

		self._report_setup_run(json!({
			"logs": error_msg,
			"status": Status::Failed,
			"started_at": started_at,
			"completed_at": completed_at,
		}))
		.await;

		self.abort(&error_msg)
	}

	async fn _report_setup_run(&self, payload: serde_json::Value) {
		let Some(url) = self.report_setup_run_url.as_ref() else {
			return;
		};

		let result = reqwest::Client::new()
			.post(url.clone())
			.json(&payload)
			.send()
			.await
			.and_then(|res| res.error_for_status());

		if result.is_err() {
			tracing::error!("failed to report setup run: {:?}", result);
		}
	}

	fn abort(&mut self, message: &str) -> Result<(), anyhow::Error> {
		self.should_exit = true;

		tracing::error!(message);
		bail!(message.to_string());
	}
}

async fn signal_pod_readiness() {
	if let Err(err) = tokio::fs::create_dir_all("/var/run/cog").await {
		tracing::error!("Failed to create cog runtime state directory: {err}");
		return;
	}

	if let Err(error) = tokio::fs::File::create("/var/run/cog/ready").await {
		tracing::error!("Failed to signal cog is ready: {error}");
	}
}
