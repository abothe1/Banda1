mod data_field;
mod preference;
pub mod category;

pub use self::data_field::DataField;
pub use self::preference::Preference;

pub trait FieldId {
	type Cat: category::Category;
	fn id(self) -> u16;
}
