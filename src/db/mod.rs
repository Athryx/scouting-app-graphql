use std::env;
use std::ops::Deref;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use juniper::executor::Context;

use crate::prelude::*;

mod schema;
mod user;
mod team;
mod data;
mod form;

type ConnType = Pool<ConnectionManager<PgConnection>>;

pub struct Connection(ConnType);

impl Connection {
	pub fn new(url: String) -> Self {
		let manager = ConnectionManager::new(url);
		Connection(Pool::new(manager).expect("could not create database connection pool"))
	}

	pub fn from_env() -> Self {
		// ignore failure
		dotenv().ok();
	
		let db_url = env::var("DATABASE_URL").expect("DATABSE_URL enviroment variable was not set");

		Self::new(db_url)
	}
}

impl Deref for Connection {
	type Target = ConnType;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Context for Connection {}
