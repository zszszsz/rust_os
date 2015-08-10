
#[macro_use]
extern crate syscalls;

pub trait WaitController
{
	fn get_count(&self) -> usize;
	fn populate(&self, cb: &mut FnMut(::syscalls::WaitItem));
	fn handle(&mut self, events: &[::syscalls::WaitItem]);
}

pub fn idle_loop(items: &mut [&mut WaitController])
{
	let mut objects = Vec::new();
	loop {
		let count = items.iter().fold(0, |sum,ctrlr| sum + ctrlr.get_count());
		objects.reserve( count );

		for ctrlr in items.iter() {
			ctrlr.populate(&mut |wi| objects.push(wi));
		}

		::syscalls::threads::wait(&mut objects, !0);

		let mut ofs = 0;
		for ctrlr in items.iter_mut()
		{
			let num = ctrlr.get_count();
			ctrlr.handle( &objects[ofs .. ofs + num] );
			ofs += num;
		}

		objects.clear();
	}
}

