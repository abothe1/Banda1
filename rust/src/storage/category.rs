pub trait Category : Copy {
	fn id() -> u8;
	fn name() -> &'static str;
}

macro_rules! category {
   ($($name:ident = $num:expr),*) => {
   	$(
	   	#[derive(Debug, Clone, Copy)]
	   	pub struct $name;

	   	impl Category for $name {
	   		fn id() -> u8 { $num }
	   		fn name() -> &'static str { stringify!($name) }
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