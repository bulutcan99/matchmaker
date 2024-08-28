use std::sync::Arc;

use crate::core::port::user::UserManagement;

#[derive(Debug, Clone)]
pub struct AuthHandler<S>
where
    S: UserManagement,
{
    pub user_service: Arc<S>,
}

impl<S> AuthHandler<S>
where
    S: UserManagement,
{
    pub fn new(user_management: Arc<S>) -> Self {
        Self {
            user_service: user_management,
        }
    }
}
