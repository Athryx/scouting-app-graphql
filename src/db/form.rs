use super::team::Team;
use super::schema::forms;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Team)]
pub struct Form {
	id: u64,
	team_id: u64,
	name: String,
}
