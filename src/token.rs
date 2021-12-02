use std::time::{SystemTime, Duration};
use std::env;
use std::fmt::{self, Formatter, Display, Debug};

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error};
use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;

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

#[derive(Debug)]
pub enum JwtError {
	TokenError(Error),
	NotAuthenticated,
}

impl From<Error> for JwtError {
	fn from(error: Error) -> Self {
		Self::TokenError(error)
	}
}

impl Display for JwtError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Debug::fmt(self, f)
	}
}

pub type JwtResult<T> = Result<T, JwtError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
	// expiration time, automatically checked by jwt library
	exp: u64,
	// user id
	id: i64,
	// user name
	name: String,
}

impl UserToken {
	pub fn from_user(user: DbUser) -> Self {
		UserToken {
			exp: SystemTime::now()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap().as_secs()
				+ VALID_TIME.as_secs(),
			id: user.id(),
			name: user.name().to_owned(),
		}
	}

	pub fn decode_from(token: &str) -> JwtResult<Self> {
		let validation = Validation::default();
		decode(token, &DECODE_KEY, &validation).map(|token| token.claims)
			.map_err(JwtError::TokenError)
	}

	pub fn encode(&self) -> JwtResult<String> {
		let header = Header::default();
		encode(&header, self, &ENCODE_KEY).map_err(JwtError::TokenError)
	}

	pub fn id(&self) -> i64 {
		self.id
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}

#[derive(Debug)]
pub enum UserTokenGuard {
	User(UserToken),
	None,
}

impl UserTokenGuard {
	pub fn is_authenticated(&self) -> bool {
		matches!(self, Self::User(_))
	}

	pub fn data(&self) -> JwtResult<&UserToken> {
		match self {
			Self::User(data) => Ok(data),
			Self::None => Err(JwtError::NotAuthenticated),
		}
	}
}

#[async_trait]
impl<'r> FromRequest<'r> for UserTokenGuard {
	type Error = JwtError;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let token = request.headers().get("token").next();
		match token {
			Some(token) => match UserToken::decode_from(token) {
				Ok(data) => Outcome::Success(UserTokenGuard::User(data)),
				Err(error) => Outcome::Failure((Status::Unauthorized, error)),
			},
			None => Outcome::Success(UserTokenGuard::None),
		}
	}
}
