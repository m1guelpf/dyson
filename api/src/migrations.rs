use ensemble::migrations::{Error, Migration, Schema};

use crate::models::WebhookEvent;

#[derive(Debug, Default)]
pub struct CreateUsersTable;

#[ensemble::async_trait]
impl Migration for CreateUsersTable {
	async fn up(&self) -> Result<(), Error> {
		Schema::create("users", |table| {
			table.uuid();
			table.string("username").unique(true);
			table.timestamps();
		})
		.await
	}

	async fn down(&self) -> Result<(), Error> {
		Schema::drop("users").await
	}
}

#[derive(Debug, Default)]
pub struct CreateTokensTable;

#[ensemble::async_trait]
impl Migration for CreateTokensTable {
	async fn up(&self) -> Result<(), Error> {
		Schema::create("api_tokens", |table| {
			table.string("token").primary(true);
			table.string("name");
			table.timestamps();

			table.foreign_uuid("user_id");
		})
		.await
	}

	async fn down(&self) -> Result<(), Error> {
		Schema::drop("api_tokens").await
	}
}

#[derive(Debug, Default)]
pub struct CreatePredictionsTable;

#[ensemble::async_trait]
impl Migration for CreatePredictionsTable {
	async fn up(&self) -> Result<(), Error> {
		Schema::create("predictions", |table| {
			table.uuid();
			table.string("version");
			table.string("status");
			table.json("input");
			table.text("logs").nullable(true);
			table.json("error").nullable(true);
			table.json("output").nullable(true);
			table.json("metrics").nullable(true);
			table.string("webhook_url").nullable(true);
			table
				.json("webhook_filter")
				.default(Vec::<WebhookEvent>::new());

			table.timestamps();
			table.timestamp("started_at").nullable(true);
			table.timestamp("completed_at").nullable(true);

			table.foreign_uuid("user_id");
		})
		.await
	}

	async fn down(&self) -> Result<(), Error> {
		Schema::drop("predictions").await
	}
}
