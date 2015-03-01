//
//
//
use _common::{Option,Some,None};
use core::ptr::PtrExt;
use _common::Send;
use core::ops::FnOnce;
use lib::mem::Box;

pub use self::queue::Queue;
pub use self::vec_map::VecMap;
pub use self::vec::Vec;
pub use self::string::String;

pub mod thunk;

//pub mod clone;

pub mod mem;
#[macro_use]
pub mod queue;
pub mod vec;
#[macro_use]
pub mod string;

pub mod vec_map;

pub mod num
{
	use core::num::Int;
	pub fn round_up<T: Int>(val: T, target: T) -> T
	{
		return (val + target - Int::one()) / target * target;
	}
}

pub mod collections
{
	pub trait MutableSeq<T>
	{
		fn push(&mut self, t: T);
		fn pop(&mut self) -> ::core::option::Option<T>;
	}
}

//pub struct LazyStatic<T>(pub Option<Box<T>>);
pub struct LazyStatic<T>(pub Option<T>);

impl<T> LazyStatic<T>
{
	pub fn prep<Fcn: FnOnce()->T>(&mut self, fcn: Fcn) {
		if self.0.is_none() {
			self.0 = Some(fcn());
		}
	}
	
	/// A fully unsafe prep function that is only valid to call when you _know_ that no races will occur
	pub unsafe fn prep_unsafe<Fcn: FnOnce()->T>(&self, fcn: Fcn) {
		assert!( self.0.is_none() );
		let mut_self: &mut LazyStatic<T> = ::core::mem::transmute(self);
		log_debug!("prep_unsafe {:p}", mut_self);
		mut_self.prep(fcn)
	}
}
impl<T> ::core::ops::Deref for LazyStatic<T>
{
	type Target = T;
	fn deref(&self) -> &T {
		&*self.0.as_ref().expect("Dereferencing LazyStatic without initialising")
	}
}
impl<T> ::core::ops::DerefMut for LazyStatic<T>
{
	fn deref_mut(&mut self) -> &mut T {
		&mut *self.0.as_mut().expect("Dereferencing LazyStatic without initialising")
	}
}

/// An equivalemnt of Option<*const T> which cannot be NULL
pub struct OptPtr<T>(pub *const T);
unsafe impl<T: Send> Send for OptPtr<T> {}
/// An equivalemnt of Option<*mut T> which cannot be NULL
pub struct OptMutPtr<T>(pub *mut T);
unsafe impl<T: Send> Send for OptMutPtr<T> {}

impl<T> OptPtr<T>
{
	pub fn is_none(&self) -> bool {
		self.0.is_null()
	}
	pub fn is_some(&self) -> bool {
		!self.0.is_null()
	}
	pub fn unwrap(&self) -> *const T {
		assert!( !self.0.is_null() );
		self.0
	}
	pub unsafe fn as_ref(&self) -> Option<&T> {
		if (self.0).is_null() {
			None
		}
		else {
			Some(&*self.0)
		}
	}
	pub unsafe fn as_mut(&self) -> OptMutPtr<T> {
		::core::mem::transmute(self)
	}
	/// This is HIGHLY unsafe
	pub unsafe fn as_mut_ref(&self) -> Option<&mut T> {
		::core::mem::transmute(self.as_ref())
	}
}

impl<T> OptMutPtr<T>
{
	pub fn is_none(&self) -> bool {
		self.0.is_null()
	}
	pub fn is_some(&self) -> bool {
		!self.0.is_null()
	}
	pub fn unwrap(&self) -> *mut T {
		assert!( !self.0.is_null() );
		self.0
	}
	pub unsafe fn as_ref(&self) -> Option<&mut T> {
		if (self.0).is_null() {
			None
		}
		else {
			Some(&mut *self.0)
		}
	}
}

pub trait UintBits
{
	fn bit(&self, idx: u8) -> Self;
	fn bits(&self, idx: u8, idx2: u8) -> Self;
}

impl UintBits for u16 {
	fn bit(&self, idx: u8) -> u16 { (*self >> idx as usize) & 1 }
	fn bits(&self, idx: u8, idx2: u8) -> u16 {
		(*self >> idx as usize) & ((1 << (idx2 - idx) as usize)-1)
	}
}

/// Printing helper for raw strings
pub struct RawString<'a>(pub &'a [u8]);

impl<'a> ::core::fmt::Debug for RawString<'a>
{
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result
	{
		try!(write!(f, "b\""));
		for &b in self.0
		{
			match b
			{
			b'\\' => try!(write!(f, "\\\\")),
			b'\n' => try!(write!(f, "\\n")),
			b'\r' => try!(write!(f, "\\r")),
			b'"' => try!(write!(f, "\\\"")),
			b'\0' => try!(write!(f, "\\0")),
			// ASCII printable characters
			32...127 => try!(write!(f, "{}", b as char)),
			_ => try!(write!(f, "\\x{:02x}", b)),
			}
		}
		try!(write!(f, "\""));
		::core::result::Result::Ok( () )
	}
}

// vim: ft=rust

