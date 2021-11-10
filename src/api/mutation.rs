use diesel::insert_into;

use crate::prelude::*;
use crate::db::Connection;
use crate::db::user::{DbUser, NewUser};
use super::user::User;

pub struct RootMutation;

#[graphql_object(Context = Connection)]
impl RootMutation {
	fn create_user(context: &Connection, name: String, password: String) -> FieldResult<User> {
		use schema::users;

		let new_user = NewUser::new(&name, &password);

		let db_user: DbUser = insert_into(users::table)
			.values(&new_user)
			.get_result(&context.get()?)?;

		Ok(db_user.into())
	}
}
