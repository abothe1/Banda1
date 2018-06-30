use key::TotalKey;

#[derive(Debug)]
pub struct User {
	username: String,
	password: u32,
	email: String,
	phone: String,
	total_key: TotalKey
}
