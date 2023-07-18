use aide::axum::ApiRouter;

mod auth;
mod docs;
mod system;
mod v1;

pub fn handler() -> ApiRouter {
	ApiRouter::new()
		.merge(docs::handler())
		.merge(system::handler())
		.merge(auth::handler())
		.merge(v1::handler())
}
