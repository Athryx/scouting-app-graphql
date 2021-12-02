use std::sync::{Mutex, MutexGuard};
use juniper::executor::Context as ContextTrait;

use crate::prelude::*;
use crate::token::{UserToken, UserTokenGuard, JwtResult};
use crate::db::ConnType;


pub struct Context {
	// ConnType is not sync
	db: Mutex<ConnType>,
	token: UserTokenGuard,
}

impl Context {
	pub fn new(db: ConnType, token: UserTokenGuard) -> Self {
		Context {
			db: Mutex::new(db),
			token,
		}
	}

	pub fn db(&self) -> MutexGuard<ConnType> {
		self.db.lock().unwrap()
	}

	pub fn token(&self) -> JwtResult<&UserToken> {
		self.token.data()
	}
}

impl ContextTrait for Context {}
