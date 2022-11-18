use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{self, dtos};

const SWAGGER_URL: &str = "/swagger/*tail";
const OPENAPI_URL: &str = "/api-doc/openapi.json";

#[derive(OpenApi)]
#[openapi(
    paths(
        api::diag::echo,
        api::setup::setup,
        api::auth::signin::signin,
        api::roles::view::get_by_id,
    ),
    components(
        schemas(dtos::setup::RootAccountDetails,
                dtos::auth::UserCredentials,
                dtos::auth::TokenPair,
                dtos::roles::RoleDto,
                dtos::roles::PermissionDto,
        )
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
