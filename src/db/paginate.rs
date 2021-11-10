use diesel::query_builder::{AstPass, QueryFragment, Query, AsQuery};
use diesel::expression::SelectableExpression;
use diesel::query_source::Table;
use diesel::query_dsl::RunQueryDsl;
use diesel::QueryResult;
use diesel::sql_types::BigInt;
use diesel::pg::{Pg, PgConnection};

pub trait Paginate: AsQuery + Sized {
	fn paginate<T, C>(self, id_column: C, page_id: i64) -> Paginated<Self::Query>
	where
		T: Table,
		C: SelectableExpression<T> + ToString,
	{
		Paginated {
			query: self.as_query(),
			// TODO: find a better way to put the column into the table
			id: id_column.to_string(),
			page_id,
			per_page: DEFAULT_PER_PAGE,
		}
	}
}

impl<T: AsQuery> Paginate for T {}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, QueryId)]
pub struct Paginated<T> {
	query: T,
	// FIXME: this might be slow, figure out how to use an &str here
	id: String,
	page_id: i64,
	per_page: i64,
}

impl<T> Paginated<T> {
	pub fn per_page(self, per_page: i64) -> Self {
		Paginated {
			per_page,
			.. self
		}
	}
}

impl<T> QueryFragment<Pg> for Paginated<T>
where
	T: QueryFragment<Pg>
{
	// code from: https://diesel.rs/guides/extending-diesel.html
	fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
		out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
		self.query.walk_ast(out.reborrow())?;
		out.push_sql(") as paged_query_with LIMIT ");
		out.push_bind_param::<BigInt, _>(&self.per_page)?;
		out.push_sql(" WHERE ");
		out.push_identifier(&self.id)?;
		out.push_sql(" >= ");
		out.push_bind_param::<BigInt, _>(&self.page_id)?;
		out.push_sql(" ORDER BY ");
		out.push_identifier(&self.id)?;
		out.push_sql(" ASC ");
		Ok(())
	}
}

impl<T: Query> Query for Paginated<T> {
	// TODO: figure out if this is right
	type SqlType = T::SqlType;
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}
