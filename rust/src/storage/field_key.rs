use super::{DataField, FieldId, Category};

#[derive(Debug)]
pub struct FieldKey<F: FieldId>(Vec<DataField<F>>);

pub struct FieldKeyIter<'a, F: FieldId + 'a>(&'a mut FieldKey<F>);

impl<F: FieldId> FieldKey<F> {
	#[inline]
	pub fn new() -> FieldKey<F> {
		FieldKey(Vec::new())
	}
	#[inline]
	pub fn with_capacity(cap: usize) -> FieldKey<F> {
		FieldKey(Vec::with_capacity(cap))
	}

	#[inline]
	pub fn push(&mut self, field: DataField<F>) {
		self.0.push(field)
	}
	
	#[inline]
	pub fn pop(&mut self) -> Option<DataField<F>> {
		self.0.pop()
	}

	#[inline]
	pub fn iter(&mut self) -> FieldKeyIter<F> {
		FieldKeyIter(self)
	}
}

impl<'a, F: FieldId + 'a> Iterator for FieldKeyIter<'a, F> {
	type Item = DataField<F>;
	fn next(&mut self) -> Option<DataField<F>> {
		(self.0).0.pop()
	}
}