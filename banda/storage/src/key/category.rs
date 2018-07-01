use super::{Field, field};
pub trait Category : Copy {
	type Field: Field;
	fn id() -> u8;
	fn assert_from(id: u8) -> Self;
}

macro_rules! category {
   ($(struct $name:ident<Field=$field:ident> = $id:expr;)*) => {
   	$(
	   	#[derive(Debug, Clone, Copy)]
	   	pub struct $name;
	   	impl Category for $name {
	   		type Field = field::$field;
	   		#[inline]
	   		fn id() -> u8 { $id }
	   		#[inline]
	   		fn assert_from(id: u8) -> $name {
	   			debug_assert_eq!(id, $id);
	   			$name
	   		}
	   	}
   	)*
   }
}

category! {
	struct Goals<Field = Goal> = 0;
	struct Genres<Field = Genre> = 1;
	struct Wants<Field = Instrument> = 2;
	struct Plays<Field = Instrument> = 3;
}
