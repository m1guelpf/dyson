use axum::{Extension, Json};
use cog_core::http::Response as PredictionResponse;

use crate::{
	server::{errors::ErrorResponse, Response},
	state::{Event, State},
};

pub async fn handle(
	Extension(state): Extension<State>,
	Json(payload): Json<PredictionResponse>,
) -> Result<Response, ErrorResponse> {
	let state = state.lock().await;
	match state.events_queue.send(Event::Webhook(payload)).await {
		Ok(_) => Ok(Response::ok()),
		Err(_) => Err(ErrorResponse::service_unavailable(
			"cannot receive webhooks: queue is full or closed",
		)),
	}
}
