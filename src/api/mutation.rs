use diesel::insert_into;

use crate::prelude::*;
use crate::db::user::{DbUser, NewUser};
use crate::db::team::{DbTeam, NewTeam, Member, NewMember, MemberType};
use super::user::User;
use super::team::Team;

pub struct RootMutation;

#[graphql_object(Context = Context)]
impl RootMutation {
	/// Create a new user
	fn create_user(context: &Context, name: String, password: String) -> FieldResult<User> {
		use schema::users;

		let new_user = NewUser::new(&name, &password);

		let db_user: DbUser = insert_into(users::table)
			.values(&new_user)
			.get_result(&*context.db())?;

		Ok(db_user.into())
	}

	/// Create a new team with the user token as the owner
	fn create_team(context: &Context, name: String) -> FieldResult<Team> {
		use schema::{teams, members};

		let token = context.token()?;

		let new_team = NewTeam::new(&name);

		let db_team: DbTeam = insert_into(teams::table)
			.values(&new_team)
			.get_result(&*context.db())?;

		let new_member = NewMember::new(token.id(), db_team.id(), MemberType::Owner);

		insert_into(members::table)
			.values(&new_member)
			.execute(&*context.db())?;

		Ok(db_team.into())
	}
}
