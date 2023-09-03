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
	RequestPartsExt, TypedHeader,
};
use ensemble::Model;

use crate::{
	errors::RouteError,
	models::{ApiToken, User},
};

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct AuthenticatedUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser {
	type Rejection = RouteError;

	async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
		let TypedHeader(Authorization(bearer)) = parts
			.extract::<TypedHeader<Authorization<Bearer>>>()
			.await
			.map_err(|_| RouteError::unauthorized())?;

		let mut token = ApiToken::find(bearer.token().to_string())
			.await
			.map_err(|_| RouteError::unauthorized())?;

		Ok(Self(token.user().await.unwrap().clone()))
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
