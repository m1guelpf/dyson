use axum::{Extension, Json};

use crate::{
	server::{errors::ErrorResponse, Response},
	state::{Event, State},
};

#[derive(Debug, serde::Deserialize)]
pub struct SetSuffixRequest {
	suffix: String,
}

pub async fn set(
	Extension(state): Extension<State>,
	Json(payload): Json<SetSuffixRequest>,
) -> Result<Response, ErrorResponse> {
	let mut state = state.lock().await;

	if state.queue_suffix.is_some() {
		return Err(ErrorResponse::conflict(
			"cannot set queue suffix: suffix is already set",
		));
	}

	if state.prediction_in_progress {
		return Err(ErrorResponse::service_unavailable(
			"cannot set queue suffix: a prediction is in progress",
		));
	}

	match state
		.events_queue
		.send(Event::Hotswap(Some(payload.suffix.clone())))
		.await
	{
		Ok(_) => {
			state.queue_suffix = Some(payload.suffix);

			Ok(Response::accepted())
		},
		Err(_) => Err(ErrorResponse::service_unavailable(
			"cannot set queue suffix: queue is full or closed",
		)),
	}
}

pub async fn clear(Extension(state): Extension<State>) -> Result<Response, ErrorResponse> {
	let mut state = state.lock().await;

	if state.queue_suffix.is_none() {
		return Err(ErrorResponse::conflict(
			"cannot clear queue suffix: no suffix exists",
		));
	}

	match state.events_queue.send(Event::Hotswap(None)).await {
		Ok(_) => {
			state.queue_suffix = None;
			Ok(Response::accepted())
		},
		Err(_) => Err(ErrorResponse::service_unavailable(
			"cannot clear queue suffix: queue is full or closed",
		)),
	}
}
