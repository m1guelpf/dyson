use aide::{
	openapi::{Parameter, ParameterData, ParameterSchemaOrContent, PathStyle, SchemaObject},
	operation::add_parameters,
	OperationInput,
};
use axum::{
	async_trait,
	extract::{FromRequestParts, Path},
	http::request::Parts,
	RequestPartsExt,
};
use ensemble::{query::Error as EnsembleError, Model};

use crate::{errors::RouteError, models::Prediction};

use super::AuthenticatedUser;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct AuthenticatedPrediction(pub Prediction);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedPrediction {
	type Rejection = RouteError;

	async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
		let Path(id) = parts.extract::<Path<String>>().await?;
		let AuthenticatedUser(user) = parts.extract::<AuthenticatedUser>().await?;

		let prediction = Prediction::find(id.parse().map_err(|_| RouteError::bad_request())?)
			.await
			.map_err(|e| match e {
				EnsembleError::NotFound => RouteError::not_found(),
				_ => RouteError::internal_error(),
			})?;

		if prediction.user != user {
			return Err(RouteError::not_found());
		}

		Ok(Self(prediction))
	}
}

impl OperationInput for AuthenticatedPrediction {
	#[allow(clippy::default_trait_access)]
	fn operation_input(ctx: &mut aide::gen::GenContext, operation: &mut aide::openapi::Operation) {
		let s = ctx.schema.subschema_for::<String>();

		add_parameters(
			ctx,
			operation,
			[Parameter::Path {
				parameter_data: ParameterData {
					explode: None,
					example: None,
					required: true,
					deprecated: None,
					examples: Default::default(),
					extensions: Default::default(),
					name: "prediction_id".to_string(),
					description: Some("The ID of the prediction".to_string()),
					format: ParameterSchemaOrContent::Schema(SchemaObject {
						json_schema: s,
						example: None,
						external_docs: None,
					}),
				},
				style: PathStyle::Simple,
			}],
		);
	}
}
