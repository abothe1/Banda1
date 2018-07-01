use mysql;

#[derive(Debug)]
pub enum Error {
	Mysql(mysql::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;


impl From<mysql::Error> for Error {
	fn from(err: mysql::Error) -> Error {
		Error::Mysql(err)
	}
}