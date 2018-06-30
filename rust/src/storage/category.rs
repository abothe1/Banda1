pub trait Category : Copy {
	fn id() -> u8;
}

macro_rules! category {
   ($($name:ident = $num:expr),*) => {
   	$(
	   	#[derive(Debug, Clone, Copy)]
	   	pub struct $name;

	   	impl Category for $name {
	   		fn id() -> u8 { $num }
	   	}
   	)*
   }
}

category!{
	Goals = 0,
	Genres = 1,
	IntsWanted = 2,
	IntsPlayed = 3
}