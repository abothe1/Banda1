use db::{Result, IntoMysql};

use mysql::{self, Pool};

#[derive(Debug)]
pub struct Database {
	database: String,
	pool: Pool
}

fn prepend_colons(strings: &[&str]) -> String {
	let mut result = String::with_capacity(strings.len() + 2 * (strings.len() - 1));
	result.push(':');
	result.push_str(strings[0]);

	for string in &strings[1..] {
		result.push_str(", :");
		result.push_str(string);
	} 
	result
}

impl Database {
	#[inline]
	pub fn from_user_port(database: &str, user: &str, port: u16) -> mysql::Result<Database> {
		Database::new(database, &format!("mysql://{}:@localhost:{}", user, port))
	}

	#[inline]
	pub fn new(database: &str, conn: &str) -> mysql::Result<Database> {
		Ok(Database{
			database: database.to_string(),
			pool: Pool::new(conn)?
		})
	}


	pub fn insert<T: IntoMysql>(&self, table: &str, obj: T) -> Result<()> {
		let insert_stmt = format!("INSERT INTO {database}.{table} ({columns}) VALUES ({values})", 
			database = self.database,
			table = table,
			columns = T::columns().join(","),
			values = prepend_colons(T::columns())
		);

		let mut stmt = self.pool.prepare(insert_stmt)?;
		stmt.execute(obj.params())?;

		Ok(())
	}
}

