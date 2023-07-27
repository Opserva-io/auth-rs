use crate::services::permission::permission_service::PermissionService;
use crate::services::role::role_service::RoleService;
use crate::services::user::user_service::UserService;

pub mod permission;
pub mod role;
pub mod user;

#[derive(Clone)]
pub struct Services {
    pub permission_service: PermissionService,
    pub role_service: RoleService,
    pub user_service: UserService,
}

impl Services {
    pub fn new(
        permission_service: PermissionService,
        role_service: RoleService,
        user_service: UserService,
    ) -> Services {
        Services {
            permission_service,
            role_service,
            user_service,
        }
    }
}
