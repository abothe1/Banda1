use key::TotalKey;
use db::IntoMysql;
use mysql::Params;

#[derive(Debug)]
pub struct User {
	username: String,
	password: u32,
	email: String,
	phone: Option<String>,
	total_key: TotalKey,
}

impl User {
	pub fn new(username: &str, password: u32, email: &str, phone: Option<&str>) -> User {
		User {
			username: username.to_string(),
			password,
			email: email.to_string(),
			phone: phone.map(ToString::to_string),
			total_key: TotalKey::new()
		}
	}
}

impl IntoMysql for User {
	fn columns() -> &'static [&'static str] {
		&[ "username", "password", "email", "phone", "total_key" ]
	}

	fn params(&self) -> Params {
		Params::from(params! {
			"username"  => &self.username,
			"password"  => &self.password,
			"email"     => &self.email,
			"phone"     => &self.phone,
			"total_key" => &self.total_key
		})
	}

}