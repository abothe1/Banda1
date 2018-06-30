use super::{FieldId, Category, Preference};
use std::fmt::{self, Debug, Formatter};

use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DataField<F: FieldId>(u16, PhantomData<F>);

impl<F: FieldId + Debug> Debug for DataField<F> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result  {
		f.debug_tuple("DataField")
			.field(&self.field_id())
			.field(&F::Category::name())
			.field(&self.pref())
			.finish()
	}
}


const MAX_FIELD_ID: u16 = 0b111_111_111_111_1; // (1 << 13) - 1
const MAX_CATEGORY_ID: u8 = 0b111;


impl<F: FieldId> DataField<F> {
	pub fn new(field_id: F, pref: Preference) -> DataField<F> {
		debug_assert!( field_id.id() <= MAX_FIELD_ID, "invalid field id `{}`", field_id.id() );
		debug_assert!( F::Category::id() <= MAX_CATEGORY_ID, "invalid cat id `{}`", F::Category::id());

		let mut inner = field_id.id() << 6;
		inner |= (F::Category::id() as u16) << 3;
		inner |= u8::from(pref) as u16;

		DataField(inner, PhantomData)
	}

	#[inline]
	pub fn pref(self) -> Preference {
		Preference::from((self.0 & 0x111) as u8)
	}

	#[inline]
	pub fn field_id(self) -> F {
		F::try_from(self.0 >> 6).expect("Invalid field id")
	}
 
}
