use super::team::DbTeam;
use super::schema::forms;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(DbTeam, foreign_key = "team_id")]
pub struct Form {
	id: u64,
	team_id: u64,
	name: String,
}
