use std::time::Duration;

use hyper::StatusCode;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use tokio::{sync::mpsc, time::sleep};
use url::Url;

use crate::state::Event;

/// How often to healthcheck initially.
const DEFAULT_POLL_INTERVAL: Duration = Duration::from_millis(100);

pub enum Actions {
	Stop,
	RequestStatus,
	SetInterval(Duration),
}

pub struct HealthChecker {
	url: Url,
	state: HealthCheck,
	interval: Duration,
	tx: mpsc::Sender<Actions>,
	rx: mpsc::Receiver<Actions>,
	client: ClientWithMiddleware,
	events_queue: mpsc::Sender<Event>,
}

impl HealthChecker {
	pub fn new(events_queue: mpsc::Sender<Event>, base_url: Url) -> Self {
		let (tx, rx) = mpsc::channel(1);
		let client = ClientBuilder::new(Client::new())
			.with(RetryTransientMiddleware::new_with_policy(
				ExponentialBackoff::builder().build_with_max_retries(6),
			))
			.build();

		Self {
			tx,
			rx,
			client,
			events_queue,
			state: HealthCheck::unknown(),
			interval: DEFAULT_POLL_INTERVAL,
			url: base_url.join("/health-check").unwrap(),
		}
	}

	pub fn agent(&self) -> Agent {
		Agent {
			tx: self.tx.clone(),
		}
	}

	pub async fn run(&mut self) {
		loop {
			tokio::select! {
				action = self.rx.recv() => {
					let Some(action) = action else {
						break
					};

					match action {
						Actions::Stop => break,
						Actions::SetInterval(interval) => self.interval = interval,
						Actions::RequestStatus => {
							self.check(true).await;
						},
					}
				},
				_ = sleep(self.interval) => {
					self.check(false).await;
				}
			}
		}
	}

	async fn check(&mut self, force_update: bool) {
		let status = self.get_status().await;

		if force_update || self.state != status {
			self.state = status.clone();

			self.events_queue
				.send(Event::Healthcheck(status))
				.await
				.unwrap();
		}
	}

	async fn get_status(&self) -> HealthCheck {
		let response = match self.client.get(self.url.clone()).send().await {
			Ok(response) => response,
			Err(_) => return HealthCheck::unknown(),
		};

		if response.status() != StatusCode::OK {
			return HealthCheck::unknown();
		}

		response
			.json::<HealthCheck>()
			.await
			.unwrap_or_else(|_| HealthCheck::unknown())
	}
}

pub struct Agent {
	tx: mpsc::Sender<Actions>,
}

impl Agent {
	pub async fn stop(&self) -> Result<(), mpsc::error::SendError<Actions>> {
		self.tx.send(Actions::Stop).await
	}

	pub async fn set_interval(
		&self,
		interval: Duration,
	) -> Result<(), mpsc::error::SendError<Actions>> {
		self.tx.send(Actions::SetInterval(interval)).await
	}

	pub async fn request_status(&self) -> Result<(), mpsc::error::SendError<Actions>> {
		self.tx.send(Actions::RequestStatus).await
	}
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Health {
	Unknown,
	Starting,
	Ready,
	Busy,
	SetupFailed,
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
pub struct HealthCheck {
	pub status: Health,
	pub setup: Option<serde_json::Value>,
}

impl HealthCheck {
	fn unknown() -> Self {
		Self {
			setup: None,
			status: Health::Unknown,
		}
	}
}
