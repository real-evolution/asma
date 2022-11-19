use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api;

const SWAGGER_URL: &str = "/swagger/*tail";
const OPENAPI_URL: &str = "/api-doc/openapi.json";

#[derive(OpenApi)]
#[openapi(
    paths(
        // diagnostics
        api::diag::echo,

        // setup
        api::setup::setup,

        // auth
        api::auth::signin::signin,

        // roles
        api::roles::view::get_all,
        api::roles::view::get_by_id,
        api::roles::add::add,
        api::roles::add::add_permission,
        api::roles::update::update,
        api::roles::remove::remove,
        api::roles::remove::remove_permission,

        // users
        api::users::view::get_by_id,
        api::users::view::get_all,
        api::users::add::add,
        api::users::remove::remove,
    ),
    components(
        schemas(
            // setup
            api::setup::dtos::RootAccountDetails,

            // auth
            api::auth::dtos::UserCredentials,
            api::auth::dtos::TokenPair,

            // roles
            api::roles::dtos::RoleDto,
            api::roles::dtos::PermissionDto,
            api::roles::dtos::RoleWithPermissionsDto,
            api::roles::dtos::AddRoleDto,
            api::roles::dtos::AddPermissionDto,
            api::roles::dtos::UpdateRoleDto,

            // users
            api::users::dtos::UserDto,
            api::users::dtos::AddUserDto,
        ),
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
