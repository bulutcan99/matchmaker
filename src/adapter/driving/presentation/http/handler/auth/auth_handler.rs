use crate::core::port::user::UserManagement;

#[derive(Debug, Copy, Clone)]
pub struct AuthHandler<S>
where
    S: UserManagement + 'static,
{
    pub user_service: S,
}

impl<S> AuthHandler<S>
where
    S: UserManagement + 'static,
{
    pub fn new(user_management: S) -> Self {
        Self {
            user_service: user_management,
        }
    }
}
