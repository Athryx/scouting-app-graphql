use std::env;
use std::ops::Deref;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use crate::prelude::*;

pub mod schema;
pub mod paginate;
pub mod user;
pub mod team;
pub mod data;
pub mod form;

pub type ConnPoolType = Pool<ConnectionManager<PgConnection>>;
pub type ConnType = PooledConnection<ConnectionManager<PgConnection>>;

pub struct Connection(ConnPoolType);

impl Connection {
	pub fn new(url: String) -> Self {
		let manager = ConnectionManager::new(url);
		Connection(Pool::new(manager).expect("could not create database connection pool"))
	}

	pub fn from_env() -> Self {
		let db_url = env::var("DATABASE_URL").expect("DATABSE_URL enviroment variable was not set");

		Self::new(db_url)
	}
}

impl Deref for Connection {
	type Target = ConnPoolType;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
