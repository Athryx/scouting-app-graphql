use crate::prelude::*;
use crate::db::team::DbTeam;

#[derive(GraphQLObject)]
pub struct Team {
	id: I64,
	name: String,
}

impl From<DbTeam> for Team {
	fn from(team: DbTeam) -> Self {
		Team {
			id: team.id().into(),
			name: team.name().to_owned(),
		}
	}
}
