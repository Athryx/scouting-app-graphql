use super::user::DbUser;
use super::schema::{teams, members};

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "teams"]
pub struct DbTeam {
	id: i64,
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

#[derive(Debug, Insertable)]
#[table_name = "teams"]
pub struct NewTeam<'a> {
	name: &'a str,
}

impl<'a> NewTeam<'a> {
	pub fn new(name: &'a str) -> Self {
		NewTeam {
			name,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
#[PgType = "member_type"]
#[DieselType = "MemberTypeMapping"]
#[derive(GraphQLEnum)]
pub enum MemberType {
	Owner,
	Admin,
	Member,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(DbUser, foreign_key = "user_id")]
#[belongs_to(DbTeam, foreign_key = "team_id")]
#[table_name = "members"]
pub struct DbMember {
	id: i64,
	user_id: i64,
	team_id: i64,
	#[column_name = "mtype"]
	pub member_type: MemberType,
}

impl DbMember {
	pub fn id(&self) -> i64 {
		self.id
	}

	pub fn member_type(&self) -> MemberType {
		self.member_type
	}
}

#[derive(Debug, Insertable)]
#[table_name = "members"]
pub struct NewMember {
	user_id: i64,
	team_id: i64,
	#[column_name = "mtype"]
	member_type: MemberType,
}

impl NewMember {
	pub fn new(user_id: i64, team_id: i64, member_type: MemberType) -> Self {
		NewMember {
			user_id,
			team_id,
			member_type,
		}
	}
}
