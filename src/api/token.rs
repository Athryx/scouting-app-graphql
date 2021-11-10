use std::time::{SystemTime, Duration};
use std::env;

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Result as JwtResult};

use crate::prelude::*;
use crate::db::user::DbUser;

// how long token is valid for
// valid for a year
const VALID_TIME: Duration = Duration::new(60 * 60 * 365, 0);

lazy_static::lazy_static! {
	static ref ENCODE_KEY: EncodingKey = EncodingKey::from_base64_secret(
		&env::var("JWT_SECRET").expect("no jwt private key"))
		.expect("invalid jwt private key format, must be base64 encoded");

	static ref DECODE_KEY: DecodingKey<'static> = DecodingKey::from_base64_secret(
		&env::var("JWT_SECRET").expect("no jwt private key"))
		.expect("invalid jwt private key format, must be base64 encoded");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTokenData {
	// expiration time, automatically checked by jwt library
	exp: u64,
	// user id
	id: i64,
	// user name
	name: String,
}

impl UserTokenData {
	pub fn from_user(user: DbUser) -> Self {
		UserTokenData {
			exp: SystemTime::now()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap().as_secs()
				+ VALID_TIME.as_secs(),
			id: user.id(),
			name: user.name().to_owned(),
		}
	}

	pub fn decode_from(token: &UserToken) -> JwtResult<Self> {
		let validation = Validation::default();
		decode(&token.0, &DECODE_KEY, &validation).map(|token| token.claims)
	}

	pub fn encode(&self) -> JwtResult<UserToken> {
		let header = Header::default();
		encode(&header, self, &ENCODE_KEY).map(UserToken)
	}

	pub fn id(&self) -> i64 {
		self.id
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}

#[derive(Debug, Clone, GraphQLScalarValue)]
#[graphql(description = "A JSON Web Token for an authenticated user")]
pub struct UserToken(String);
