use crate::services::jwt::jwt_service::JwtService;
use crate::services::permission::permission_service::PermissionService;
use crate::services::role::role_service::RoleService;
use crate::services::user::user_service::UserService;

pub mod jwt;
pub mod permission;
pub mod role;
pub mod user;

#[derive(Clone)]
pub struct Services {
    pub permission_service: PermissionService,
    pub role_service: RoleService,
    pub user_service: UserService,
    pub jwt_service: JwtService,
}

impl Services {
    /// # Summary
    ///
    /// Create a new instance of Services.
    ///
    /// # Arguments
    ///
    /// * `permission_service` - The PermissionService.
    /// * `role_service` - The RoleService.
    /// * `user_service` - The UserService.
    /// * `jwt_service` - The JwtService.
    ///
    /// # Returns
    ///
    /// A new instance of Services.
    pub fn new(
        permission_service: PermissionService,
        role_service: RoleService,
        user_service: UserService,
        jwt_service: JwtService,
    ) -> Services {
        Services {
            permission_service,
            role_service,
            user_service,
            jwt_service,
        }
    }
}
