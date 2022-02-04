use diesel::{insert_into, update};

use crate::prelude::*;
use super::member::Member;
use crate::db::team::{DbTeam, DbMember, NewMember, MemberType};
use crate::db::user::{DbUser};

pub struct Team {
	id: I64,
	name: String,
}

#[graphql_object(Context = Context)]
impl Team {
	pub fn id(&self, _context: &Context) -> I64 {
		self.id
	}

	pub fn name(&self, _context: &Context) -> &str {
		&self.name
	}

	pub fn members(&self, context: &Context, offset: Option<I64>, limit: Option<I64>) -> FieldResult<Vec<Member>> {
		use schema::members;
		use schema::users;

		let offset = offset.unwrap_or(0.into());
		let limit = limit.unwrap_or(i64::MAX.into());

		let members = members::table.order_by(members::id.asc())
			.filter(members::id.ge(offset.n())
				.and(members::team_id.eq(self.id.n())))
			.inner_join(users::table)
			.limit(limit.n())
			.load::<(DbMember, DbUser)>(&*context.db())?;

		let mut out = Vec::with_capacity(members.len());

		for (member, user) in members {
			out.push(Member::new(member.id().into(), member.member_type(), user.into()));
		}

		Ok(out)
	}

	// FIXME: this doesn't feel very graphql like, but it is easy to do
	// FIXME: check authentication
	/// Adds the user as a member to this team and returns the member id
	/// If the user is already a member, this will update the member type
	pub fn add_member(&self, context: &Context, id: I64, member_type: MemberType) -> FieldResult<I64> {
		use schema::members;

		let user_id = id.into();
		let team_id = self.id.into();

		let new_member = NewMember::new(user_id, team_id, member_type);

		// TODO: figure out if there is a way to do this in one sql query for more performance
		let member = members::table.filter(members::id.eq(user_id).and(members::team_id.eq(team_id)))
			.first::<DbMember>(&*context.db());

		match member {
			Ok(mut member) => {
				if member.member_type() != member_type {
					member.member_type = member_type;
					update(members::table).set(&member)
						.execute(&*context.db())?;

					Ok(member.id().into())
				} else {
					Ok(member.id().into())
				}
			},
			Err(_) => {
				let new_member = insert_into(members::table)
					.values(&new_member)
					.get_result::<DbMember>(&*context.db())?;
				Ok(new_member.id().into())
			},
		}
	}
}

impl From<DbTeam> for Team {
	fn from(team: DbTeam) -> Self {
		Team {
			id: team.id().into(),
			name: team.name().to_owned(),
		}
	}
}
