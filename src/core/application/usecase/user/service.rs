use crate::core::domain::entity::user::User;
use crate::core::port::auth::TokenMaker;
use crate::core::port::storage::Storage;

pub struct UserService<T, K>
where
	T: TokenMaker,
	K: Storage<User>,
{
	token_handler: T,
	user_repository: K,
}