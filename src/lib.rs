// Dylaedin Operating System
// Caedin and Dylan
// 10/1/2025
#![no_std]
use core::arch::asm;

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({

	});
}
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

// ///////////////////////////////////
// / LANGUAGE STRUCTURES / FUNCTIONS
// ///////////////////////////////////
#[no_mangle]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(_p) = info.location() {
		println!(
				 "line {}, file {}: {}",
				 _p.line(),
				 _p.file(),
				 info.message().unwrap()
		);
	}
	else {
		println!("no information available.");
	}
	abort();
}
#[no_mangle]
extern "C"
fn abort() -> ! {
	loop {
		unsafe {
			asm!("wfi", options(nomem, nostack, preserves_flags));
		}
	}
}

#[no_mangle]
extern "C"
fn kmain() {
	
	print_str("Hello, World! Check out the Dylaedin Operating System!");
	loop {}
}

fn print_char(c: char) {
	unsafe {
		//create a pointer to 0x10000000;
		let ptr = 0x10000000 as *mut u32;
		//set to 0x64
		ptr.write_volatile(c as u32);
	}
}

fn print_str(s: &str) {
	for c in s.chars() {
		print_char(c);
	}
}

