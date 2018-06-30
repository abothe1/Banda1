#[repr(u8)]
pub enum Preference { Hate, Dislike, Meh, Neutral, Ok, Like, Love }

pub trait Category : From<u8>{
	fn id() -> u8; // _needs_ to be 7 bits.
}

pub trait DataId : From<u16> {
	type Category: Category;
	fn id(self) -> u16;
}


mod data_point;

pub use self::data_point::DataPoint;


#[repr(u8)]
enum Genre { Jaz, Rock, Edm }
impl Category for Category {
	fn id() -> Category { 0 }
}

fn foo() {


}









