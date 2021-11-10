use std::num::ParseIntError;
use std::cmp::min;

use juniper::{Value, InputValue, ParseError, ParseScalarResult, parser::{ScalarToken, Token}};

// an unsigned 64 bit integer that can be used in graphql, since there is no unsigned 64 bit integer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U64(u64);

impl U64 {
	pub fn n(&self) -> u64 {
		self.0
	}
}

impl From<u64> for U64 {
	fn from(n: u64) -> Self {
		U64(n)
	}
}

impl Into<u64> for U64 {
	fn into(self) -> u64 {
		self.0
	}
}

impl TryFrom<String> for U64 {
	type Error = ParseIntError;

	fn try_from(val: String) -> Result<Self, Self::Error> {
		val.parse::<u64>().map(U64)
	}
}

#[graphql_scalar(
	name = "UInt64",
	description = "An unsigned 64 bit integer")]
impl<S> GraphQLScalar for U64
	where S: ScalarValue
{
	fn resolve(&self) -> Value {
		Value::scalar(self.0.to_string())
	}

	fn from_input_value(value: &InputValue) -> Option<Self> {
		match *value {
			InputValue::Scalar(ref val) => {
				val.as_int().map(|int| int.try_into().ok()).flatten()
					.or_else(|| val.as_string().map(|string| string.parse::<u64>().ok()).flatten())
					.map(U64)
			},
			_ => None,
		}
	}

	fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
		match value {
			ScalarToken::String(value) | ScalarToken::Int(value) => Ok(S::from(value.to_owned())),
			_ => Err(ParseError::UnexpectedToken(Token::Scalar(value))),
		}
	}
}

// an signed 64 bit integer that can be used in graphql, since there is no unsigned 64 bit integer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct I64(i64);

impl I64 {
	pub fn n(&self) -> i64 {
		self.0
	}
}

impl From<i64> for I64 {
	fn from(n: i64) -> Self {
		I64(n)
	}
}

impl Into<i64> for I64 {
	fn into(self) -> i64 {
		self.0
	}
}

impl TryFrom<String> for I64 {
	type Error = ParseIntError;

	fn try_from(val: String) -> Result<Self, Self::Error> {
		val.parse::<i64>().map(I64)
	}
}

#[graphql_scalar(
	name = "Int64",
	description = "A signed 64 bit integer")]
impl<S> GraphQLScalar for I64
	where S: ScalarValue
{
	fn resolve(&self) -> Value {
		Value::scalar(self.0.to_string())
	}

	fn from_input_value(value: &InputValue) -> Option<Self> {
		match *value {
			InputValue::Scalar(ref val) => {
				val.as_int().map(|int| int.into())
					.or_else(|| val.as_string().map(|string| string.parse::<i64>().ok()).flatten())
					.map(I64)
			},
			_ => None,
		}
	}

	fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
		match value {
			ScalarToken::String(value) | ScalarToken::Int(value) => Ok(S::from(value.to_owned())),
			_ => Err(ParseError::UnexpectedToken(Token::Scalar(value))),
		}
	}
}
