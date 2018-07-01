use super::{DataField, Category};
use std::fmt::{self, Debug, Formatter};
use std::slice::Iter;
use db::Bytes;

#[derive(Clone)]
pub struct FieldKey<C>(Vec<DataField<C>>);

impl<C: Category + Debug> Debug for FieldKey<C>
			where C::Field: Debug {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("FieldKey").field(&self.0).finish()
	}
}

impl<C: Clone> Default for FieldKey<C> {
	#[inline]
	fn default() -> FieldKey<C> {
		FieldKey::new()
	}
}

impl<C, I: Into<Vec<DataField<C>>>> From<I> for FieldKey<C> {
	fn from(iter: I) -> FieldKey<C> {
		FieldKey(iter.into())
	}
}

impl<C> From<FieldKey<C>> for Bytes {
	fn from(key: FieldKey<C>) -> Bytes {
		let mut bytes = Bytes::with_capacity(key.0.len() * 2);
		for data_field in key.0 {
			bytes.extend(Bytes::from(data_field));
		}

		bytes
	}
}


impl<C: Clone> FieldKey<C> {
	#[inline]
	pub fn new() -> FieldKey<C> {
		FieldKey(Vec::new())
	}
	#[inline]
	pub fn with_capacity(cap: usize) -> FieldKey<C> {
		FieldKey(Vec::with_capacity(cap))
	}

	#[inline]
	pub fn push(&mut self, field: DataField<C>) {
		self.0.push(field)
	}

	#[inline]
	pub fn push_all(&mut self, fields: &[DataField<C>]) {
		self.0.extend_from_slice(fields)
	}
	
	#[inline]
	pub fn pop(&mut self) -> Option<DataField<C>> {
		self.0.pop()
	}

	#[inline]
	pub fn iter(&self) -> Iter<DataField<C>> {
		self.0.iter()
	}
}
