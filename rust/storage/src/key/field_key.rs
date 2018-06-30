use super::{DataField, Category};
use std::fmt::{self, Debug, Formatter};
use std::slice::Iter;

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
	pub fn iter(&mut self) -> Iter<DataField<C>> {
		self.0.iter()
	}
}
