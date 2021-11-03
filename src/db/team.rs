use super::user::User;
use super::schema::{teams, members};

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key = "owner_id")]
pub struct Team {
	id: u64,
	owner_id: u64,
	name: String,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Team)]
pub struct Member {
	id: u64,
	user_id: u64,
	team_id: u64,
	admin: bool,
}
