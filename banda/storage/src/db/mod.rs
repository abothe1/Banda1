mod database;
mod error;

pub use self::database::Database;
pub use self::error::{Error, Result};

pub(crate) type Bytes = Vec<u8>;