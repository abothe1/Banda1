use std::fmt::{self, Debug, Formatter};
pub struct Preference(u8);


impl Debug for Preference {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result  {
		Debug::fmt(&self.0, f)
	}
}

impl Preference {
	#[inline]
	pub fn none() -> Preference {
		Preference::new(0)
	}

	#[inline]
	pub fn new(pref: u8) -> Preference {
		assert!(pref <= 7, "invalid preference `{}`", pref);
		Preference(pref)
	}
}

impl From<u8> for Preference {
	#[inline]
	fn from(pref: u8) -> Preference {
		Preference::new(pref)
	}
}

impl From<Preference> for u8 {
	#[inline]
	fn from(pref: Preference) -> u8 {
		pref.0
	}
}