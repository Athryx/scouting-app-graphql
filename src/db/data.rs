use super::schema::{datasets, entries};
use super::team::Team;
use super::form::Form;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Team)]
#[belongs_to(Form)]
pub struct Dataset {
	id: u64,
	team_id: u64,
	form_id: u64,
	name: String,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Dataset)]
#[table_name = "entries"]
pub struct Entry {
	id: u64,
	dataset_id: u64,
}
