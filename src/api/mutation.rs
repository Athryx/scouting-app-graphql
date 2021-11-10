use diesel::insert_into;

use crate::prelude::*;
use crate::db::Connection;
use crate::db::user::{DbUser, NewUser};
use crate::db::team::DbTeam;
use super::user::User;

pub struct RootMutation;

#[graphql_object(Context = Connection)]
impl RootMutation {
	/// Create a new user
	fn create_user(context: &Connection, name: String, password: String) -> FieldResult<User> {
		use schema::users;

		let new_user = NewUser::new(&name, &password);

		let db_user: DbUser = insert_into(users::table)
			.values(&new_user)
			.get_result(&context.get()?)?;

		Ok(db_user.into())
	}

	/// Create a new team with the user token as the owner
	fn create_team(context: &Connection, token: UserToken, name: String) -> FieldResult<Team> {
		use schema::teams;
		todo!();
	}
}
