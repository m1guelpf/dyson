use std::env;

use anyhow::Result;
use reqwest::Client;
use url::Url;

use crate::models::{Prediction, WebhookEvent};

pub struct WebhookSender {
	client: Client,
}

impl WebhookSender {
	pub fn new() -> Result<Self> {
		Ok(Self {
			client: Client::builder()
				.user_agent(format!(
					"dyson-api/{}",
					option_env!("GIT_REV").unwrap_or(env!("STATIC_BUILD_DATE"))
				))
				.build()?,
		})
	}

	#[allow(dead_code)]
	pub async fn starting(&self, prediction: &Prediction) -> Result<()> {
		if !Self::should_send(prediction, WebhookEvent::Start) {
			return Ok(());
		}

		self.send(prediction.webhook_url.clone().unwrap(), prediction)
			.await?;

		Ok(())
	}

	pub async fn finished(&self, prediction: &Prediction) -> Result<()> {
		if !Self::should_send(prediction, WebhookEvent::Completed) {
			return Ok(());
		}

		self.send(prediction.webhook_url.clone().unwrap(), prediction)
			.await?;

		Ok(())
	}

	fn should_send(req: &Prediction, event: WebhookEvent) -> bool {
		req.webhook_url.is_some()
			&& (req.webhook_filter.is_empty() || req.webhook_filter.contains(&event))
	}

	async fn send(&self, url: Url, res: &Prediction) -> Result<reqwest::Response, reqwest::Error> {
		tracing::trace!(prediction = ?res, "Sending webhook to {url}");

		self.client.post(url).json(&res).send().await
	}
}
