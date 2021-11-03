use crate::db::Connection;

pub struct RootMutation;

#[graphql_object(Context = Connection)]
impl RootMutation {
	fn tmp() -> i32 {
		todo!();
	}
}
