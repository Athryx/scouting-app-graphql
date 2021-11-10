use super::user::DbUser;
use super::schema::{teams, members};

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(DbUser, foreign_key = "owner_id")]
#[table_name = "teams"]
pub struct DbTeam {
	id: i64,
	owner_id: i64,
	name: String,
}

impl DbTeam {
	pub fn id(&self) -> i64 {
		self.id
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(DbUser, foreign_key = "user_id")]
#[belongs_to(DbTeam, foreign_key = "team_id")]
pub struct Member {
	id: i64,
	user_id: i64,
	team_id: i64,
	admin: bool,
}
