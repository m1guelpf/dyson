#![allow(dead_code)]
use aide::OperationOutput;
use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{self, Debug, Display, Formatter};

pub struct RouteError {
	status_code: StatusCode,
	extra_data: Option<Value>,
	error: Option<anyhow::Error>,
	public_error_message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RouteErrorOutput {
	pub error: String,

	#[serde(flatten, skip_serializing_if = "Option::is_none")]
	pub extra_data: Option<Value>,
}

impl RouteError {
	#[must_use]
	pub fn unauthorized() -> Self {
		Self::from_status(StatusCode::UNAUTHORIZED)
	}

	#[must_use]
	pub fn not_found() -> Self {
		Self::from_status(StatusCode::NOT_FOUND)
	}

	#[must_use]
	pub fn bad_request() -> Self {
		Self::from_status(StatusCode::BAD_REQUEST)
	}

	#[must_use]
	pub fn unprocessable_entity() -> Self {
		Self::from_status(StatusCode::UNPROCESSABLE_ENTITY)
	}

	#[must_use]
	pub fn internal_error() -> Self {
		Self::from_status(StatusCode::INTERNAL_SERVER_ERROR)
	}

	#[must_use]
	pub fn conflict() -> Self {
		Self::from_status(StatusCode::CONFLICT)
	}

	#[must_use]
	pub fn from_status(status_code: StatusCode) -> Self {
		Self {
			status_code,
			..Self::default()
		}
	}
}

impl RouteError {
	#[must_use]
	pub fn set_status_code(self, status_code: StatusCode) -> Self {
		Self {
			status_code,
			..self
		}
	}

	#[must_use]
	pub fn set_error(self, error: anyhow::Error) -> Self {
		Self {
			error: Some(error),
			..self
		}
	}

	#[must_use]
	pub fn set_data(self, extra_data: Value) -> Self {
		Self {
			error: self.error,
			extra_data: Some(extra_data),
			status_code: self.status_code,
			public_error_message: self.public_error_message,
		}
	}

	#[must_use]
	pub fn set_message(self, public_error_message: &str) -> Self {
		Self {
			public_error_message: Some(public_error_message.to_string()),
			..self
		}
	}

	pub fn message(&self) -> &str {
		if let Some(public_error_message) = self.public_error_message.as_ref() {
			return public_error_message;
		}

		status_code_to_public_message(self.status_code())
	}

	pub const fn status_code(&self) -> StatusCode {
		self.status_code
	}
}

impl Default for RouteError {
	fn default() -> Self {
		Self {
			error: None,
			extra_data: None,
			public_error_message: None,
			status_code: StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
}

impl IntoResponse for RouteError {
	fn into_response(self) -> Response {
		let status = self.status_code();
		let extra_data = self.extra_data;
		let error = self.public_error_message.map_or_else(
			|| status_code_to_public_message(status).to_string(),
			|public_error_message| public_error_message,
		);

		let output = RouteErrorOutput { error, extra_data };
		let body = Json(output);

		(status, body).into_response()
	}
}

impl OperationOutput for RouteError {
	type Inner = RouteErrorOutput;
}

impl Debug for RouteError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}, {:?}", self.message(), self.error)
	}
}

impl Display for RouteError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.message())
	}
}

impl<FE: Into<anyhow::Error>> From<FE> for RouteError {
	fn from(error: FE) -> Self {
		Self {
			status_code: StatusCode::INTERNAL_SERVER_ERROR,
			error: Some(error.into()),
			..Self::default()
		}
	}
}

const fn status_code_to_public_message(status_code: StatusCode) -> &'static str {
	match status_code {
		StatusCode::CONFLICT => "The request is not allowed",
		StatusCode::UNAUTHORIZED => "You are not authorised to access this endpoint",
		StatusCode::NOT_FOUND => "The resource was not found",
		StatusCode::BAD_REQUEST => "Bad request made",
		StatusCode::FORBIDDEN => "Request is forbidden",
		StatusCode::IM_A_TEAPOT => "I'm a teapot",
		StatusCode::TOO_MANY_REQUESTS => "Too many requests",
		StatusCode::BAD_GATEWAY => "Bad gateway",
		StatusCode::SERVICE_UNAVAILABLE => "Service unavailable",
		StatusCode::UNPROCESSABLE_ENTITY => "Unprocessable entity",
		StatusCode::GATEWAY_TIMEOUT => "Gateway timeout",
		StatusCode::INTERNAL_SERVER_ERROR => "An unexpected error occurred",
		_ => "An unknown error occurred",
	}
}
