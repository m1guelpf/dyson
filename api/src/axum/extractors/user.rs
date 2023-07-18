use aide::{
	openapi::{HeaderStyle, Parameter, ParameterData, ParameterSchemaOrContent, SchemaObject},
	operation::add_parameters,
	OperationInput,
};
use axum::{
	async_trait,
	extract::FromRequestParts,
	headers::{authorization::Bearer, Authorization},
	http::request::Parts,
	Extension, RequestPartsExt, TypedHeader,
};
use axum_route_error::RouteError;
use std::sync::Arc;

use crate::db::{api_token, user, PrismaClient};

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct AuthenticatedUser(pub user::Data);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser {
	type Rejection = RouteError;

	async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
		let TypedHeader(Authorization(bearer)) = parts
			.extract::<TypedHeader<Authorization<Bearer>>>()
			.await
			.map_err(|_| RouteError::new_unauthorized())?;

		let Extension(prisma) = parts.extract::<Extension<Arc<PrismaClient>>>().await?;

		let token = prisma
			.api_token()
			.find_unique(api_token::token::equals(bearer.token().to_string()))
			.with(api_token::user::fetch())
			.exec()
			.await?
			.ok_or_else(RouteError::new_unauthorized)?;

		Ok(Self(token.user().unwrap().clone()))
	}
}

impl OperationInput for AuthenticatedUser {
	#[allow(clippy::default_trait_access)]
	fn operation_input(ctx: &mut aide::gen::GenContext, operation: &mut aide::openapi::Operation) {
		let s = ctx.schema.subschema_for::<String>();

		add_parameters(
			ctx,
			operation,
			[Parameter::Header {
				parameter_data: ParameterData {
					explode: None,
					example: None,
					required: true,
					deprecated: None,
					examples: Default::default(),
					extensions: Default::default(),
					name: "Authorization".to_string(),
					description: Some("Bearer token".to_string()),
					format: ParameterSchemaOrContent::Schema(SchemaObject {
						json_schema: s,
						example: None,
						external_docs: None,
					}),
				},
				style: HeaderStyle::Simple,
			}],
		);
	}
}
