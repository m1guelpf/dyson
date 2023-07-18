use aide::axum::{routing::get, ApiRouter};

use crate::axum::extractors::AuthenticatedUser;

pub fn handler() -> ApiRouter {
	ApiRouter::new().api_route("/me", get(get_user))
}

#[allow(clippy::unused_async)]
async fn get_user(AuthenticatedUser(user): AuthenticatedUser) -> String {
	format!(
		"Hi @{}! You are authenticated correctly, welcome to Dyson :)",
		user.username
	)
}
