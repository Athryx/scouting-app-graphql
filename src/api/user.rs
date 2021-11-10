use crate::prelude::*;
use crate::db::user::DbUser;

#[derive(GraphQLObject)]
pub struct User {
	id: I64,
	name: String,
}

impl From<DbUser> for User {
	fn from(user: DbUser) -> Self {
		User {
			id: user.id().into(),
			name: user.name().to_owned(),
		}
	}
}
