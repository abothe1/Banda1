mod database;
mod error;

pub use self::database::Database;
pub use self::error::{Error, Result};

pub(crate) type Bytes = Vec<u8>;

pub trait IntoMysql {
	fn columns() -> &'static [&'static str];
	fn params(&self) -> ::mysql::Params;
	fn default_db_table() -> Option<&'static str> { None }
}
