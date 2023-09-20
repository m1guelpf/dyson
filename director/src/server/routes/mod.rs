use axum::{
	routing::{get, post},
	Router,
};

mod queue_suffix;
mod webhook;

pub fn router() -> Router {
	Router::new()
		.route("/", get(|| async { "Hello, world!" }))
		.route("/webhook", post(webhook::handle))
		.route(
			"/queue-suffix",
			post(queue_suffix::set).delete(queue_suffix::clear),
		)
}
