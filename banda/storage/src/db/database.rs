use user::User;
use key::{FieldKey, category::*};
use mysql::{self, Pool};

#[derive(Debug)]
pub struct Database {
	pool: Pool
}


impl Database {
	#[inline]
	pub fn from_user_port(user: &str, port: u16) -> mysql::Result<Database> {
		Database::new(&format!("mysql://{}:@localhost:{}", user, port))
	}

	#[inline]
	pub fn new(conn: &str) -> mysql::Result<Database> {
		Ok(Database{
			pool: Pool::new(conn)?
		})
	}
}

impl Database {
	fn insert_user_main(&self, user: &User) -> mysql::Result<u64> {
		let insert_stmt = format!("INSERT INTO banda_users.users ({}) VALUES ({})",
			User::columns_str(), User::values_str()
		);

		let mut stmt = self.pool.prepare(insert_stmt)?;
		let user_id = stmt.execute(user.as_params())?.last_insert_id();

		Ok(user_id)
	}

	fn insert_genres(&self, user_id: u64, genres: &FieldKey<Genres>) -> mysql::Result<()> {
		let mut stmt = self.pool.prepare("INSERT INTO banda_users.user_genres (user_id, genre, level) VALUES (:user_id, :genre, :level)")?;
		for data_field in genres.iter() {
			stmt.execute(params!{
				"user_id" => user_id,
				"genre" => u16::from(data_field.field_id()),
				"level" => u8::from(data_field.level())
			})?;
		}

		Ok(())
	}

	fn insert_plays(&self, user_id: u64, instrs: &FieldKey<Plays>) -> mysql::Result<()> {
		let mut stmt = self.pool.prepare("INSERT INTO banda_users.user_plays (user_id, instr, level) VALUES (:user_id, :instr, :level)")?;
		for data_field in instrs.iter() {
			stmt.execute(params!{
				"user_id" => user_id,
				"instr" => u16::from(data_field.field_id()),
				"level" => u8::from(data_field.level())
			})?;
		}
		Ok(())
	}

	fn insert_wants(&self, user_id: u64, instrs: &FieldKey<Wants>) -> mysql::Result<()> {
		let mut stmt = self.pool.prepare("INSERT INTO banda_users.user_wants (user_id, instr, level) VALUES (:user_id, :instr, :level)")?;
		for data_field in instrs.iter() {
			stmt.execute(params!{
				"user_id" => user_id,
				"instr" => u16::from(data_field.field_id()),
				"level" => u8::from(data_field.level())
			})?;
		}
		Ok(())
	}

	fn insert_goals(&self, user_id: u64, instrs: &FieldKey<Goals>) -> mysql::Result<()> {
		let mut stmt = self.pool.prepare("INSERT INTO banda_users.user_goals (user_id, goal) VALUES (:user_id, :goal)")?;
		for data_field in instrs.iter() {
			stmt.execute(params!{
				"user_id" => user_id,
				"goal" => u16::from(data_field.field_id())
			})?;
		}

		Ok(())
	}



	pub fn insert_user(&self, user: &User) -> mysql::Result<()> {
		let user_id = self.insert_user_main(user)?;
		self.insert_genres(user_id, &user.total_key.genres)?;
		self.insert_plays(user_id, &user.total_key.plays)?;
		self.insert_wants(user_id, &user.total_key.wants)?;
		self.insert_goals(user_id, &user.total_key.goals)?;
		Ok(())
	}
}



