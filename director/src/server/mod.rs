use axum::{response::IntoResponse, Extension};
use hyper::StatusCode;
use std::{env, net::SocketAddr};
use tokio::task::JoinHandle;

use crate::{shutdown::Shutdown, state::State};

mod errors;
mod routes;

pub fn start(shutdown: Shutdown, app_state: State) -> JoinHandle<Result<(), hyper::Error>> {
	let server = routes::router()
		.layer(shutdown.extension())
		.layer(Extension(app_state));

	let addr = SocketAddr::from((
		[0, 0, 0, 0],
		env::var("PORT")
			.unwrap_or_else(|_| "4900".to_string())
			.parse()
			.unwrap(),
	));
	tracing::info!("🛠️ Starting Director on http://{}", addr);

	tokio::spawn(async move {
		axum::Server::bind(&addr)
			.serve(server.into_make_service())
			.with_graceful_shutdown(shutdown.handle())
			.await
	})
}

#[derive(Debug, serde::Serialize)]
struct Response {
	status: &'static str,

	#[serde(skip)]
	status_code: StatusCode,
}

impl Response {
	pub fn ok() -> Self {
		Self {
			status: "ok",
			status_code: StatusCode::OK,
		}
	}

	pub fn accepted() -> Self {
		Self {
			status: "accepted",
			status_code: StatusCode::ACCEPTED,
		}
	}
}

impl IntoResponse for Response {
	fn into_response(self) -> axum::response::Response {
		(self.status_code, axum::Json(self)).into_response()
	}
}
