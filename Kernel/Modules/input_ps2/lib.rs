// "Tifflin" Kernel
// - By John Hodge (thePowersGang)
//
// Modules/input_ps2/lib.rs
//! PS2 Keyboard/Mouse controller
#![feature(no_std,core,linkage)]
#![no_std]
#![feature(core_slice_ext)]
#![feature(const_fn)]	// needed for lazystatic_init
#[macro_use] extern crate core;
#[macro_use] extern crate kernel;
use kernel::prelude::*;

// HACK: Requires USB to be active to ensure that emulation is off
module_define!{PS2, [DeviceManager, ACPI, GUI/*, USB*/], init}

#[derive(Debug)]
enum PS2Dev
{
	None,
	Unknown,
	Enumerating(EnumWaitState),
	Keyboard(keyboard::Dev),
	Mouse(mouse::Dev),
}
impl Default for PS2Dev { fn default() -> Self { PS2Dev::None } }

#[derive(Copy,Clone,Debug)]
enum EnumWaitState
{
	DSAck,
	IdentAck,
	IdentB1,
	IdentB2(u8),
}

mod i8042;

mod keyboard;
mod mouse;

fn init()
{
	// TODO: Support other controller types (e.g. the ARM PL050)
	i8042::init();
}

impl PS2Dev
{
	fn new_mouse(ty: mouse::Type) -> (Option<u8>, Option<PS2Dev>) {
		let (byte, dev) = mouse::Dev::new(ty);
		(byte, Some(PS2Dev::Mouse(dev)))
	}
	fn new_keyboard(ty: keyboard::Type) -> (Option<u8>, Option<PS2Dev>) {
		let (byte, dev) = keyboard::Dev::new(ty);
		(byte, Some(PS2Dev::Keyboard(dev)))
	}
	
	/// Handle a recieved byte, and optionally return a byte to be sent to the device
	pub fn recv_byte(&mut self, byte: u8) -> Option<u8> {
		log_trace!("PS2 Byte {:#x}", byte);
		let (rv, new_state): (Option<_>,Option<_>) = match *self
			{
			PS2Dev::None =>
				// TODO: Clean this section up, the OSDev.org wiki is a little hazy on the ordering
				if byte == 0xFA {
					(None, None)
				}
				else if byte == 0xAA {
					(Some(0xF5), Some(PS2Dev::Enumerating(EnumWaitState::DSAck)))
				}
				else {
					(None, None)
				},
			PS2Dev::Unknown => (None, None),
			PS2Dev::Enumerating(state) => match state
				{
				EnumWaitState::DSAck =>
					if byte == 0xFA {
						(Some(0xF2), Some(PS2Dev::Enumerating(EnumWaitState::IdentAck)))
					}
					else {
						(None, Some(PS2Dev::Unknown))
					},
				EnumWaitState::IdentAck =>
					if byte == 0xFA {
						(None, Some(PS2Dev::Enumerating(EnumWaitState::IdentB1)))
					}
					else {
						(None, Some(PS2Dev::Unknown))
					},
				EnumWaitState::IdentB1 =>
					match byte
					{
					0x00 => Self::new_mouse(mouse::Type::Std),
					0x03 => Self::new_mouse(mouse::Type::Scroll),
					0x04 => Self::new_mouse(mouse::Type::QuintBtn),
					0xAB => (None, Some(PS2Dev::Enumerating(EnumWaitState::IdentB2(byte)))),
					_ => {
						log_warning!("Unknown PS/2 device {:#02x}", byte);
						(None, Some(PS2Dev::Unknown))
						},
					},
				EnumWaitState::IdentB2(b1) =>
					match (b1,byte)
					{
					(0xAB, 0x83) => Self::new_keyboard(keyboard::Type::MF2),
					(0xAB, 0x41) => Self::new_keyboard(keyboard::Type::MF2Emul),
					(0xAB, 0xC1) => Self::new_keyboard(keyboard::Type::MF2Emul),
					_ => {
						log_warning!("Unknown PS/2 device {:#02x} {:#02x}", b1, byte);
						(None, Some(PS2Dev::Unknown))
						},
					},
				},
			PS2Dev::Keyboard(ref mut dev) => {
				(dev.recv_byte(byte), None)
				},
			PS2Dev::Mouse(ref mut dev) => {
				(dev.recv_byte(byte), None)
				},
			};
		
		if let Some(ns) = new_state
		{
			log_debug!("State transition {:?} to {:?}", *self, ns);
			*self = ns;
		}
		rv
	}
}

