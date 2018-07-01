use key::{TotalKey, DataField, category::*};
use mysql::Value;

#[derive(Debug)]
pub struct User {
	username: String,
	password: u32,
	email: String,
	phone: Option<String>,
	pub(crate) total_key: TotalKey,
}

impl User {
	pub(crate) fn columns_str() -> &'static str { "username, password, email, phone, total_key" }
	pub(crate) fn values_str() -> &'static str { ":username, :password, :email, :phone, :total_key" }
	pub(crate) fn as_params(&self) -> Vec<(String, Value)> {
		params! {
			"username"  => &self.username,
			"password"  => &self.password,
			"email"     => &self.email,
			"phone"     => &self.phone,
			"total_key" => &self.total_key
		}
	}
}

impl User {
	pub fn new(username: &str, password: u32,
              email: &str, phone: Option<&str>,
	           goals: &[DataField<Goals>], genres: &[DataField<Genres>],
	           wants: &[DataField<Wants>], plays: &[DataField<Plays>]) -> User {
		User {
			username: username.to_string(),
			password,
			email: email.to_string(),
			phone: phone.map(ToString::to_string),
			total_key: TotalKey::new(goals, genres, wants, plays)
		}
	}
}
