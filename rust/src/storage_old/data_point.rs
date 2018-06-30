use super::{Category, DataId, Preference};
use std::marker::PhantomData;
// use std::fmt::{self, Debug, Formatter};


pub struct DataPoint<C>(u16, PhantomData<C>);

// impl<T> Debug for DataPoint<T> {
// 	fn fmt(&self, f: &mut Formatter) -> fmt::Result  {
// 		f.debug_tuple("DataPoint")
// 			.field(&self.id())
// 			.field(&self.category())
// 			.field(&self.data())
// 			.finish()
// 	}
// }

impl<C: Category> DataPoint<C> {
	pub fn new<I: DataId<Category=C>>(id: I, pref: Preference) -> DataPoint<C> {
		let id = id.id() << 6;
		let cat = C::id() << 3;

		DataPoint(id | cat as u16 | pref as u16, PhantomData)
	}
}









