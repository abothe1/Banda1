use super::{FieldId, category::Category, Preference};

use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct DataField<C>(u16, PhantomData<C>);


impl<C: Category> DataField<C> {
	pub fn new<F: FieldId<Cat=C>>(id: F, pref: Preference) -> DataField<C> {
		debug_assert!( id.id() <= 0b111_111_111_111_1, "invalid field id `{}`", id.id() );
		debug_assert!( C::id() <= 0b111, "invalid cat id `{}`", C::id());

		let mut inner = id.id() << 6;
		inner |= (C::id() as u16) << 3;
		inner |= u8::from(pref) as u16;

		DataField(inner, PhantomData)
	}

	pub fn pref(self) -> Preference {
		Preference::from((self.0 & 0x111) as u8)
	}
}