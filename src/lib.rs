// Dylaedin Operating System
// Caedin and Dylan
// 10/1/2025
#![no_std]
/*
	Mods
*/
mod srv;
mod dev;
mod emu;
/*
	Idk Stuff Here ;)
 */
use core::{arch::{asm}, panic::PanicInfo};
use dev::uart::Uart;
use srv::console::Console;
use dev::pci;
/*
	Globals
*/

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
		use core::fmt::Write;
		let _ = write!(crate::dev::uart::Uart::new(0x1000_0000 as *mut u8), $($args)+);
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
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(_p) = info.location() {
		println!(
				 "line {}, file {}: {}",
				 _p.line(),
				 _p.file(),
				 info.message()
		);
	}
	else {
		println!("no information available.");
	}
	loop {
		unsafe {
			asm!("wfi", options(nomem, nostack, preserves_flags));
		}
	}
}

#[no_mangle]
fn get_dts() -> u64 {
	let value: u64;
	unsafe {
		asm!(
			"mv {0}, a2",
			out(reg) value
		);
	}
	println!("device tree at: {:X}", value);
	return value;
}

#[no_mangle]
extern "C"
fn kmain() {
	// Getting the device tree from a register
	let device_tree_addr: u64 =  get_dts();
	let kernel_uart: Uart = Uart::new(0x1000_0000 as *mut u8);
	// let kconsole: Console = Console::new(kernel_uart);
	println!("Hello, World!");
	//printsizeof PCIHeader0
	println!("Size of PCIHeader0: {}", core::mem::size_of::<pci::PCIHeader0>());
	let pci = pci::PCIHeader0::get(0, 1);
	println!("Vendor ID: {:#X}", pci.header.vendor_id());
	println!("Device ID: {:#X}", pci.header.device_id());
	//print out address
	println!("Subsytem Vendor ID: {:#X}", pci.subsystem_vendor_id);
	println!("Subsytem ID: {:#X}", pci.subsystem_id);
	println!("Interrupt Line: {:#X}", pci.interrupt_line);
	println!("Interrupt Pin: {:#X}", pci.interrupt_pin);
	println!("Expansion ROM Base Address: {:#X}", pci.expansion_rom_base_address as u32);
	unsafe {
		let cap = pci.capabilities_head as *const pci::PCICapabilitiesList;
		println!("Capabilities Head: {:#X}", (*cap).capability_id());
		println!("Capabilities Next: {:#X}", (*cap).next());
		let next = pci.base_address.add((*cap).next() as usize) as *const pci::PCICapabilitiesList;
		println!("Capabilities Next: {:#X}", (*next).capability_id());
	}
	//read the value back
	// println!("Vendor ID: {:#X}", result);/
	// kconsole.listen();
	loop {}
}

