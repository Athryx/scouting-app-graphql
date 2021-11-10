use argon2::{
	password_hash::{
		rand_core::OsRng,
		PasswordHash, PasswordHasher, PasswordVerifier, SaltString
	},
	Argon2,
	Algorithm,
	Version,
	Params,
};

use lazy_static::lazy_static;

use super::schema::users;

// default parameters recomended from some website
lazy_static! {
	// NOTE: the memory argument to params is in KiB
	static ref HASHER: Argon2<'static> = Argon2::new(Algorithm::Argon2id, Version::default(), Params::new(15 * 1024, 2, 1, None).unwrap());
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "users"]
pub struct DbUser {
	id: i64,
	name: String,
	password: String,
}

impl DbUser {
	pub fn id(&self) -> i64 {
		self.id
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn check_password(&self, password: &str) -> bool {
		let bytes = password.as_bytes();
		let old_hash = PasswordHash::new(&self.name).expect("invalid password hash stored in database");

		HASHER.verify_password(bytes, &old_hash).is_ok()
	}
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
	name: &'a str,
	password: String,
}

impl<'a> NewUser<'a> {
	pub fn new(username: &'a str, password: &str) -> Self {
		let salt = SaltString::generate(&mut OsRng);
		let hashed_password = HASHER.hash_password(password.as_bytes(), &salt).unwrap().to_string();

		NewUser {
			name: username,
			password: hashed_password,
		}
	}
}
