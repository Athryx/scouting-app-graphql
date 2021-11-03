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

use super::schema::users;

use lazy_static::lazy_static;

const MIB: u32 = 1024 * 1024;

// default parameters recomended from some website
lazy_static! {
	static ref HASHER: Argon2<'static> = Argon2::new(Algorithm::Argon2id, Version::default(), Params::new(15 * MIB, 2, 1, None).unwrap());
}

#[derive(Debug, Queryable, Identifiable)]
pub struct User {
	id: u64,
	name: String,
	password: String,
}

impl User {
	pub fn id(&self) -> u64 {
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
