use super::{FieldKey, category::*};
use db::Bytes;
use mysql;

#[derive(Debug, Default, Clone)]
pub struct TotalKey {
	pub goals: FieldKey<Goals>,
	pub genres: FieldKey<Genres>,
	pub wants: FieldKey<Wants>,
	pub plays: FieldKey<Plays>
}

impl TotalKey {
	pub fn new<Gl, Gn, W, P>(goals: Gl, genres: Gn, wants: W, plays: P) -> TotalKey
				where Gl: Into<FieldKey<Goals>>,
						Gn: Into<FieldKey<Genres>>,
						W: Into<FieldKey<Wants>>,
						P: Into<FieldKey<Plays>>{
		TotalKey {
			goals: goals.into(),
			genres: genres.into(),
			wants: wants.into(),
			plays: plays.into()
		}
	}
}

impl From<TotalKey> for Bytes {
	fn from(key: TotalKey) -> Bytes {
		let TotalKey { goals, genres, wants, plays } = key;

		let mut bytes = Bytes::new();
		bytes.extend(Bytes::from(goals));
		bytes.extend(Bytes::from(genres));
		bytes.extend(Bytes::from(wants));
		bytes.extend(Bytes::from(plays));

		bytes
	}
}

impl From<TotalKey> for mysql::Value {
	fn from(key: TotalKey) -> mysql::Value {
		mysql::Value::Bytes(key.into())
	}
}


