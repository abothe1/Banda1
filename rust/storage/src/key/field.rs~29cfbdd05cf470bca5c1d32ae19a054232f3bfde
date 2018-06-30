use std::mem;

pub trait Field : Copy + From<u16> {
	fn id(self) -> u16;
}

macro_rules! field {
   ($($id:tt: pub enum $name:ident { $($var:ident),* })*) => {
   	$(
			#[derive(Debug, Copy, Clone)]
			#[repr(u16)]
			pub enum $name {
				$($var,)*
				#[doc(hidden)]
				__Last
			}

			impl Field for $name {
				fn id(self) -> u16 { $id }
			}

			impl From<u16> for $name {
				fn from(id: u16) -> $name {
					if id < $name::__Last as u16 {
						unsafe{ mem::transmute::<u16, $name>(id) }
					} else {
						panic!(concat!("Unknown field `{}` for ", stringify!($name)), id)
					}
				}
			}
		)*
   }
}

field!{
	0: pub enum Goal { BecomeFamous }
	1: pub enum Genre { Jazz, Rock, Edm, Blues }
	2: pub enum Instrument { Drums, Guitar, Piano, Trumpet }
}



