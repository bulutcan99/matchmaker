use crate::core::port::user::UserManagement;

pub struct AuthHandler<S>
where
    S: UserManagement,
{
    pub user_service: S,
}
impl<S> AuthHandler<S>
where
    S: UserManagement,
{
    pub fn new(user_management: S) -> Self {
        Self {
            user_service: user_management,
        }
    }
}
