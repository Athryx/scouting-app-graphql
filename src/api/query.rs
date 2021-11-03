use crate::db::Connection;

use crate::prelude::*;
use super::user::User;

pub struct RootQuery;

#[graphql_object(Context = Connection)]
impl RootQuery {
	fn users(context: &Connection, id: ID, len: i32) -> Vec<&User> {
		todo!();
	}
}
