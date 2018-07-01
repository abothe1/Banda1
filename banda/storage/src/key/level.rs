use std::fmt::{self, Debug, Formatter};
pub struct Level(u8);


impl Debug for Level {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result  {
		Debug::fmt(&self.0, f)
	}
}

impl Level {
	#[inline]
	pub fn none() -> Level {
		Level::new(0)
	}

	#[inline]
	pub fn new(pref: u8) -> Level {
		assert!(pref <= 7, "invalid preference level `{}`", pref);
		Level(pref)
	}
}

impl From<u8> for Level {
	#[inline]
	fn from(pref: u8) -> Level {
		Level::new(pref)
	}
}

impl From<Level> for u8 {
	#[inline]
	fn from(pref: Level) -> u8 {
		pref.0
	}
}