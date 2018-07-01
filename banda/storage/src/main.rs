extern crate banda_storage;

use banda_storage::db::Database;
use banda_storage::key::DataField;
use banda_storage::user::User;

#[repr(u8)] enum Pref { Unknown, Hate, Dislike, Meh, Netural, Ok, Like, Love } use self::Pref::*;
impl From<Pref> for ::banda_storage::key::Level { fn from(inp: Pref) -> Self { Self::from(inp as u8)}}
#[repr(u8)] enum Genres { Jazz, Rock, Blues, Exper, Class, Funk, Latin, Edm } use self::Genres::*;
impl From<Genres> for ::banda_storage::key::field::Genre { fn from(inp: Genres) -> Self { Self::from(inp as u16)}}
#[repr(u8)] enum Instrs { Drums, Guitar, Piano, Voice } use self::Instrs::*;
impl From<Instrs> for ::banda_storage::key::field::Instrument { fn from(inp: Instrs) -> Self { Self::from(inp as u16)}}

macro_rules! df {
   ($a:expr) => { DataField::new($a, 0) };
   ($a:expr, $b:expr) => { DataField::new($a, $b) }
}

fn main(){
	let sam = User::new("westerhack", 0xff, "westerhack@gmail.com", None,
		/* goals  */ &[ df!(3) ],
		/* genres */ &[ df!(Edm, Love), df!(Jazz, Meh), df!(Exper, Unknown) ],
		/* wants  */ &[ df!(Guitar), df!(Voice), df!(Piano) ],
		/* plays  */ &[ df!(Drums) ]
	);

	let kwan = User::new("kwan", 0xff, "ethan@kwan.work", None,
		/* goals  */ &[ df!(5) ],
		/* genres */ &[ df!(Edm, Like), df!(Rock, Dislike), df!(Class, Hate), df!(Blues, Ok) ],
		/* wants  */ &[ df!(Drums), df!(Guitar) ],
		/* plays  */ &[ df!(Piano), df!(Voice), df!(Drums) ]
	);


	let db = Database::from_user_port("root", 3306).expect("couldnt make db");
	db.insert_user(&sam).expect("couldnt insert sam into db");
	db.insert_user(&kwan).expect("couldnt insert kwawn into db");
}
