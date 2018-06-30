use super::{FieldKey, category::*};

#[derive(Debug, Default, Clone)]
pub struct TotalKey {
	pub goals: FieldKey<Goals>,
	pub genres: FieldKey<Genres>,
	pub wanted: FieldKey<IntsWanted>,
	pub plays: FieldKey<IntsPlayed>
}

impl TotalKey {
	#[inline]
	pub fn new() -> TotalKey {
		TotalKey::default()
	}
}
