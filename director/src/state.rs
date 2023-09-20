use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use crate::health_check::HealthCheck;

#[derive(Debug)]
pub enum Event {
	Hotswap(Option<String>),
	Healthcheck(HealthCheck),
	Webhook(cog_core::http::Response),
}

#[derive(Debug)]
pub struct AppState {
	pub queue_suffix: Option<String>,
	pub prediction_in_progress: bool,
	pub events_queue: mpsc::Sender<Event>,
}

pub type State = Arc<Mutex<AppState>>;

impl AppState {
	fn new(events_queue: mpsc::Sender<Event>) -> Self {
		Self {
			events_queue,
			queue_suffix: None,
			prediction_in_progress: false,
		}
	}
}

pub fn build(tx: mpsc::Sender<Event>) -> State {
	Arc::new(Mutex::new(AppState::new(tx)))
}
