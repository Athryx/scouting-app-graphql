use crate::prelude::*;
use crate::db::Connection;
use crate::db::user::DbUser;
use crate::db::team::DbTeam;
use super::user::User;
use super::team::Team;

pub struct RootQuery;

#[graphql_object(Context = Connection)]
impl RootQuery {
	fn users(context: &Connection, offset: I64, limit: I64) -> FieldResult<Vec<User>> {
		use schema::users;

		let users = users::table.order_by(users::id.asc())
			.filter(users::id.ge(offset.n()))
			.limit(limit.n())
			.load::<DbUser>(&context.get()?)?;

		let mut out = Vec::with_capacity(users.capacity());
		for user in users {
			out.push(user.into());
		}

		Ok(out)
	}

	// FIXME: this is very bad performance
	fn search_users(context: &Connection, name: String, limit: I64) -> FieldResult<Vec<User>> {
		use schema::users;

		// add % to both sides to match all characters
		let query_string = format!("%{}%", name);

		let users = users::table.order_by(users::id.asc())
			.filter(users::name.ilike(&query_string))
			.limit(limit.n())
			.load::<DbUser>(&context.get()?)?;

		let mut out = Vec::with_capacity(users.capacity());
		for user in users {
			out.push(user.into());
		}

		Ok(out)
	}

	fn teams(context: &Connection, offset: I64, limit: I64) -> FieldResult<Vec<Team>> {
		use schema::teams;

		let teams = teams::table.order_by(teams::id.asc())
			.filter(teams::id.ge(offset.n()))
			.limit(limit.n())
			.load::<DbTeam>(&context.get()?)?;

		let mut out = Vec::with_capacity(teams.capacity());
		for team in teams {
			out.push(team.into());
		}

		Ok(out)
	}

	// FIXME: this is very bad performance
	fn search_teams(context: &Connection, name: String, limit: I64) -> FieldResult<Vec<Team>> {
		use schema::teams;

		let query_string = format!("%{}%", name);

		let teams = teams::table.order_by(teams::id.asc())
			.filter(teams::name.ilike(&query_string))
			.limit(limit.n())
			.load::<DbTeam>(&context.get()?)?;

		let mut out = Vec::with_capacity(teams.capacity());
		for team in teams {
			out.push(team.into());
		}

		Ok(out)
	}
}
