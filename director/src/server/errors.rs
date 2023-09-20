use axum::{
	response::{IntoResponse, Response},
	Json,
};
use hyper::StatusCode;

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
	detail: String,

	#[serde(skip)]
	status: StatusCode,
}

impl ErrorResponse {
	pub fn new(status: StatusCode, detail: &str) -> Self {
		Self {
			status,
			detail: detail.to_string(),
		}
	}

	pub fn service_unavailable(detail: &str) -> Self {
		Self::new(StatusCode::SERVICE_UNAVAILABLE, detail)
	}

	pub fn conflict(detail: &str) -> Self {
		Self::new(StatusCode::CONFLICT, detail)
	}
}

impl IntoResponse for ErrorResponse {
	fn into_response(self) -> Response {
		(self.status, Json(self)).into_response()
	}
}
