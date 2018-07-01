use super::{Field, Category, Level};
use db::Bytes;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DataField<C>(u16, PhantomData<C>);

impl<C: Category + Debug> Debug for DataField<C>
			where C::Field: Debug {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result  {
		f.debug_tuple("DataField")
			.field(&self.field_id())
			.field(&self.category())
			.field(&self.level())
			.finish()
	}
}


impl<C> From<DataField<C>> for Bytes {
	fn from(data_field: DataField<C>) -> Bytes {
		vec![(data_field.0 >> 8) as u8, data_field.0 as u8]
	}
}

const MAX_FIELD_ID: u16 = 0b111_111_111_111_1; // (1 << 13) - 1

impl<C: Category> DataField<C> {
	pub fn new<F, L>(field: F, level: L) -> DataField<C>
				where F: Into<C::Field>,
						L: Into<Level> {
		let field = field.into();
		let level = level.into();
		debug_assert!( field.id() <= MAX_FIELD_ID, "invalid field id `{}`", field.id() );

		let mut inner = field.id() << 6;
		inner |= (C::id() as u16) << 3;
		inner |= u8::from(level) as u16;

		DataField(inner, PhantomData)
	}

	#[inline]
	pub fn level(self) -> Level {
		Level::from((self.0 & 0x111) as u8)
	}

	#[inline]
	pub fn field_id(self) -> C::Field {
		<C::Field>::from(self.0 >> 6)
	}

	#[inline]
	pub fn category(self) -> C {
		C::assert_from((self.0 >> 3) as u8 & 7)
	} 
}
