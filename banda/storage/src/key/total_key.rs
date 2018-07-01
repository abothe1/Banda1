use super::{FieldKey, category::*};
use db::Bytes;
use mysql;

#[derive(Debug, Default, Clone)]
pub struct TotalKey {
	pub goals: FieldKey<Goals>,
	pub genres: FieldKey<Genres>,
	pub wanted: FieldKey<IntsWanted>,
	pub played: FieldKey<IntsPlayed>
}

impl TotalKey {
	#[inline]
	pub fn new() -> TotalKey {
		TotalKey::default()
	}
}
impl From<TotalKey> for Bytes {
	fn from(key: TotalKey) -> Bytes {
		let TotalKey { goals, genres, wanted, played } = key;

		let mut bytes = Bytes::new();
		bytes.extend(Bytes::from(goals));
		bytes.extend(Bytes::from(genres));
		bytes.extend(Bytes::from(wanted));
		bytes.extend(Bytes::from(played));

		bytes
	}
}

impl From<TotalKey> for mysql::Value {
	fn from(key: TotalKey) -> mysql::Value {
		mysql::Value::Bytes(key.into())
	}
}


