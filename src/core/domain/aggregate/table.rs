use surrealdb::sql::Id;
use crate::core::domain::aggregate::user_profile::UserAggregate;

pub struct Table{
	id: Option<Id>,
	user1: UserAggregate,
	user2: UserAggregate,
}