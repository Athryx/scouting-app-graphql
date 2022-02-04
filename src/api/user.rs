use crate::prelude::*;
use super::member::MemberOf;
use crate::db::user::DbUser;
use crate::db::team::{DbTeam, DbMember};

pub struct User {
	id: I64,
	name: String,
}

#[graphql_object(Context = Context)]
impl User {
	pub fn id(&self, _context: &Context) -> I64 {
		self.id
	}

	pub fn name(&self, _context: &Context) -> &str {
		&self.name
	}

	pub fn teams(&self, context: &Context, offset: Option<I64>, limit: Option<I64>) -> FieldResult<Vec<MemberOf>> {
		use schema::members;
		use schema::teams;

		let offset = offset.unwrap_or(0.into());
		let limit = limit.unwrap_or(i64::MAX.into());

		let teams = members::table.order_by(members::id.asc())
			.filter(members::id.ge(offset.n())
				.and(members::user_id.eq(self.id.n())))
			.inner_join(teams::table)
			.limit(limit.n())
			.load::<(DbMember, DbTeam)>(&*context.db())?;

		let mut out = Vec::with_capacity(teams.len());

		for (member, team) in teams {
			out.push(MemberOf::new(member.id().into(), member.member_type(), team.into()));
		}

		Ok(out)
	}
}

impl From<DbUser> for User {
	fn from(user: DbUser) -> Self {
		User {
			id: user.id().into(),
			name: user.name().to_owned(),
		}
	}
}
