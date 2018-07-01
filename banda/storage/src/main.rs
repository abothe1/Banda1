extern crate banda_storage;

use banda_storage::db::Database;
use banda_storage::user::User;

fn main(){
	let user = User::new("westerhack", 0xff, "westerhack@gmail.com", None);
	let db = Database::from_user_port("banda_users", "root", 3306).expect("couldnt make db");
	db.insert("users", user).expect("couldnt insert into db");
}