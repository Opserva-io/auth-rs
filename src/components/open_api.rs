use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::OpenApi;
use utoipa::{openapi, Modify};

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    /// # Summary
    ///
    /// Adds the security scheme to the OpenAPI specification.
    ///
    /// # Arguments
    ///
    /// * `openapi` - A mutable reference to the OpenAPI specification.
    ///
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::web::controller::authentication::authentication_controller::login,
        crate::web::controller::authentication::authentication_controller::register,
        crate::web::controller::authentication::authentication_controller::current_user,
        crate::web::controller::health::health_controller::health,
        crate::web::controller::permission::permission_controller::create_permission,
        crate::web::controller::permission::permission_controller::find_all_permissions,
        crate::web::controller::permission::permission_controller::find_by_id,
        crate::web::controller::permission::permission_controller::update_permission,
        crate::web::controller::permission::permission_controller::delete_permission,
        crate::web::controller::role::role_controller::create,
        crate::web::controller::role::role_controller::find_all_roles,
        crate::web::controller::role::role_controller::find_by_id,
        crate::web::controller::role::role_controller::update,
        crate::web::controller::role::role_controller::delete,
        crate::web::controller::user::user_controller::create,
        crate::web::controller::user::user_controller::find_all,
        crate::web::controller::user::user_controller::find_by_id,
        crate::web::controller::user::user_controller::update,
        crate::web::controller::user::user_controller::update_self,
        crate::web::controller::user::user_controller::update_password,
        crate::web::controller::user::user_controller::admin_update_password,
        crate::web::controller::user::user_controller::delete,
        crate::web::controller::user::user_controller::delete_self,
        crate::web::controller::audit::audit_controller::find_all,
        crate::web::controller::audit::audit_controller::find_by_id,
    ),
    components(
        schemas(
            crate::errors::internal_server_error::InternalServerError,
            crate::errors::bad_request::BadRequest,
            crate::web::dto::permission::create_permission::CreatePermission,
            crate::web::dto::permission::permission_dto::PermissionDto,
            crate::web::dto::permission::update_permission::UpdatePermission,
            crate::web::controller::health::health_controller::HealthResponse,
            crate::web::dto::authentication::login_request::LoginRequest,
            crate::web::dto::authentication::login_response::LoginResponse,
            crate::web::dto::authentication::register_request::RegisterRequest,
            crate::web::dto::user::user_dto::SimpleUserDto,
            crate::web::dto::role::role_dto::SimpleRoleDto,
            crate::web::dto::permission::permission_dto::SimplePermissionDto,
            crate::web::dto::role::role_dto::RoleDto,
            crate::web::dto::role::create_role::CreateRole,
            crate::web::dto::role::update_role::UpdateRole,
            crate::web::dto::user::create_user::CreateUser,
            crate::web::dto::user::user_dto::UserDto,
            crate::web::dto::user::update_user::UpdateUser,
            crate::web::dto::user::update_user::UpdateOwnUser,
            crate::web::dto::user::update_password::UpdatePassword,
            crate::web::dto::user::update_password::AdminUpdatePassword,
            crate::web::dto::audit::audit_dto::AuditDto,
            crate::web::dto::audit::audit_dto::ActionDto,
            crate::web::dto::audit::audit_dto::ResourceIdTypeDto,
            crate::web::dto::audit::audit_dto::ResourceTypeDto,
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
