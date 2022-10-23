use crate::api;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const SWAGGER_URL: &str = "/swagger/*tail";
const OPENAPI_URL: &str = "/api-doc/openapi.json";

#[derive(OpenApi)]
#[openapi(
    paths(
        api::diag::echo,
        api::auth::signin,
    ),
    components(
        schemas(api::auth::UserCredentials, api::auth::TokenPair)
    ),
    tags((
        name = "asma",
        description = "Advanced Social Media Aggregator"
    ))
)]
struct ApiDoc;

pub fn create_swagger_ui() -> SwaggerUi {
    SwaggerUi::new(SWAGGER_URL).url(OPENAPI_URL, ApiDoc::openapi())
}
