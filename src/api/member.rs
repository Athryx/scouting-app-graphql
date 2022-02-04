use crate::prelude::*;
use super::user::User;
use super::team::Team;
use crate::db::team::MemberType;

pub struct Member {
	id: I64,
	member_type: MemberType,
	user: User,
}

#[graphql_object(Context = Context)]
impl Member {
	pub fn id(&self, _context: &Context) -> I64 {
		self.id
	}

	pub fn member_type(&self, _context: &Context) -> MemberType {
		self.member_type
	}

	pub fn user(&self, _context: &Context) -> &User {
		&self.user
	}
}

impl Member {
	pub fn new(id: I64, member_type: MemberType, user: User) -> Self {
		Member {
			id,
			member_type,
			user,
		}
	}
}

pub struct MemberOf {
	id: I64,
	member_type: MemberType,
	team: Team,
}

#[graphql_object(Context = Context)]
impl MemberOf {
	pub fn id(&self, _context: &Context) -> I64 {
		self.id
	}

	pub fn member_type(&self, _context: &Context) -> MemberType {
		self.member_type
	}

	pub fn team(&self, _context: &Context) -> &Team {
		&self.team
	}
}

impl MemberOf {
	pub fn new(id: I64, member_type: MemberType, team: Team) -> Self {
		MemberOf {
			id,
			member_type,
			team,
		}
	}
}
