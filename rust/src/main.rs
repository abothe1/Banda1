#![allow(unused)]
extern crate banda;

use banda::storage::*;

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
enum Genre { Jaz, Rock, Edm, #[doc(hidden)] __Last }

impl FieldId for Genre {
	type Category = category::Genres;
	fn id(self) -> u16 { self as u16 }
	fn try_from(id: u16) -> Option<Self> {
		if id < Genre::__Last as u16 {
			Some(unsafe { ::std::mem::transmute::<u16, Genre>(id) })
		} else {
			None
		}
	}
}

fn main(){
	let field = DataField::new(Genre::Jaz, 1.into());
	println!("{:?}", field);
}