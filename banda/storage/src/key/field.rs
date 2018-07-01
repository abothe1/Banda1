use std::mem;

pub trait Field : Copy + From<u16> {
	fn id(self) -> u16;
}

macro_rules! field {
   ($($id:tt: pub struct $name:ident; )*) => {
   	$(
			#[derive(Debug, Copy, Clone)]
			pub struct $name(u16);

			impl $name {
				#[inline]
				pub fn new(inp: u16) -> $name { $name(inp) }
			}

			impl Field for $name {
				#[inline]
				fn id(self) -> u16 { $id }
			}

			impl From<$name> for u16 {
				#[inline]
				fn from(inp: $name) -> u16 { inp.0 }
			}

			impl From<u16> for $name {
				#[inline]
				fn from(inp: u16) -> $name { $name::from(inp) }
			}
		)*
   }
}

field!{
	0: pub struct Goal;
	1: pub struct Genre;
	2: pub struct Instrument;
}



