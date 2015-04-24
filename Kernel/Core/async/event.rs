// "Tifflin" Kernel
// - By John Hodge (thePowersGang)
//
// Core/async/event.rs
//! Asynchronous event waiter
use _common::*;
use core::atomic::{AtomicBool,ATOMIC_BOOL_INIT};
use core::fmt;

/// A general-purpose wait event (when flag is set, waiters will be informed)
///
/// Only a single object can wait on this event at one time
///
/// TODO: Determine the set/reset conditions on the wait flag.
pub struct Source
{
	flag: AtomicBool,
	waiter: ::sync::mutex::Mutex<Option<::threads::SleepObjectRef>>
}

/// Event waiter
pub struct Waiter<'a>
{
	/// Event source
	source: Option<&'a Source>,
}

static s_event_none: Source = Source { flag: ATOMIC_BOOL_INIT, waiter: mutex_init!(None) };

impl Source
{
	/// Create a new event source
	pub fn new() -> Source
	{
		Source {
			flag: ATOMIC_BOOL_INIT,
			waiter: ::sync::mutex::Mutex::new(None),
		}
	}
	/// Return a wait handle for this event source
	pub fn wait<'a>(&'a self) -> Waiter<'a>
	{
		Waiter {
			source: Some(self),
			}
	}
	/// Raise the event (waking any attached waiter)
	pub fn trigger(&self)
	{
		self.flag.store(true, ::core::atomic::Ordering::Relaxed);
		self.waiter.lock().as_mut().map(|r| r.signal());
	}
}

impl<'a> fmt::Debug for Waiter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Waiter")
	}
}

impl<'a> super::PrimitiveWaiter for Waiter<'a>
{
	fn poll(&self) -> bool {
		match self.source {
		Some(r) => r.flag.load(::core::atomic::Ordering::Relaxed),
		None => true,
		}
	}
	fn run_completion(&mut self) {
		// Do nothing
	}
	fn bind_signal(&mut self, sleeper: &mut ::threads::SleepObject) -> bool {
		match self.source
		{
		Some(r) => {
			*r.waiter.lock() = Some(sleeper.get_ref());
			},
		None => {
			},
		}
		
		true
	}
}