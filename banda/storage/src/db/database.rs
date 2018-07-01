use db::{Error, Result, IntoMysql};

use mysql::{self, Pool, OptsBuilder, Opts};

#[derive(Debug)]
pub struct Database {
	pool: Pool
}

fn prepend_colons(strings: &[&str]) -> String {
	let mut result = String::with_capacity(strings.len() + 2 * (strings.len() - 1));
	result.push_str(strings[0]);

	for string in &strings[1..] {
		result.push_str(", ");
		result.push_str(string);
	} 
	result
}

impl Database {
	#[inline]
	pub fn from_user_port(user: &str, port: u16) -> mysql::Result<Database> {
		Database::new(&format!("mysql://{}:@localhost:{}", user, port))
	}

	#[inline]
	pub fn new(conn: &str) -> mysql::Result<Database> {
		Ok(Database{ pool: Pool::new(conn)? })
	}


	pub fn insert<T: IntoMysql>(&self, obj: T) -> Result<()> {
		match T::default_table() {
			Some(table) => self.insert_with_table(table, obj),
			None => Err(Error::TableRequired)
		}
	}

	pub fn insert_with_table<T: IntoMysql>(&self, table: &str, obj: T) -> Result<()> {
		let insert_stmt = format!("INSERT INTO {table} ({columns}) VALUES ({values})", 
			table = table,
			columns = T::columns().join(","),
			values = prepend_colons(T::columns())
		);

		let mut stmt = self.pool.prepare(insert_stmt)?;
		stmt.execute(obj.params())?;

		Ok(())
	}
}

