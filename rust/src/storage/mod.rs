mod data_field;
mod preference;
pub mod category;

pub use self::data_field::DataField;
pub use self::preference::Preference;

pub trait FieldId : Copy {
	type Category: category::Category;
	fn id(self) -> u16;
	fn try_from(id: u16) -> Option<Self>;
}

