mod data_field;
mod field_key;
mod preference;
pub mod category;

use self::category::Category;
pub use self::data_field::DataField;
pub use self::field_key::{FieldKey, FieldKeyIter};
pub use self::preference::Preference;

pub trait FieldId : Copy {
	type Category: category::Category;
	fn id(self) -> u16;
	fn try_from(id: u16) -> Option<Self>;
}

