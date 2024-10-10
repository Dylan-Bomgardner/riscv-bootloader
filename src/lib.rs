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
use core::{arch::asm, convert::Infallible, panic::PanicInfo};
use dev::uart::Uart;
use srv::console::Console;
use dev::pci;
use embedded_graphics::{
    mono_font::{ascii::FONT_7X14, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
    text::Text,
};
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
pub struct Mode13Display {
    base: *mut u8,
}

impl Mode13Display {
    #[inline]
    pub unsafe fn new(base: *mut u8) -> Self {
        Self { base }
    }

    #[inline(always)]
    fn _color_component_to_safe_color(c: u8) -> u8 {
        const TABLE: [u8; 256] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ];
        TABLE[c as usize]
    }

    pub fn set_pixel(&mut self, coord: Point, color: Rgb888) -> Option<()> {
        if let Ok((x @ 0..=319, y @ 0..=199)) = coord.try_into() {
            let index = x as usize + y as usize * 320;

            let r = Self::_color_component_to_safe_color(color.r());
            let g = Self::_color_component_to_safe_color(color.g());
            let b = Self::_color_component_to_safe_color(color.b());
            let color = 16 + r + g * 6 + b * 36;

            unsafe {
                self.base.add(index).write_volatile(color);
            }

            Some(())
        } else {
            None
        }
    }
}

impl DrawTarget for Mode13Display {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            self.set_pixel(coord, color);
        }
        Ok(())
    }
}

impl OriginDimensions for Mode13Display {
    fn size(&self) -> Size {
        Size::new(320, 200)
    }
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
	println!("Size of PCIHeader0: {}", core::mem::size_of::<pci::PCIDevice>());
	let pci = pci::PCIDevice::get(0, 1);
	println!("Vendor ID: {:#X}", pci.header.vendor_id);
	println!("Device ID: {:#X}", pci.header.device_id);
	println!("Class Code: {:#X}", pci.header.class_code);
	println!("Subclass: {:#X}", pci.header.subclass);
	println!("Address Range: {:#X}", pci.address_range);
	//check the first outside the address range
	// pci.header.command().set_memory_space(true);
	unsafe {
	println!("Address {:#X}", pci.read(0x10));
	}
	//print out address
	for i in 0..6 {
		println!("BAR{}: {:#X}", i, pci.bar_read(i));
	}
	//try to find bochs version
	unsafe 
	{
		println!("Bochs Version: {:#X}", pci.bar_read(0x08));
	}

	let vga = dev::vga::VGA::new(0, 1).unwrap();
	unsafe {
		vga.set_mode13();
	}
	let mut display = unsafe { Mode13Display::new(vga.fb) };

    Text::new(
        "Hello, world!",
        Point::new(4, 18),
        MonoTextStyleBuilder::new()
            .font(&FONT_7X14)
            .text_color(Rgb888::WHITE)
            .build(),
    )
    .draw(&mut display)
    .unwrap();

	//read the value back
	// println!("Vendor ID: {:#X}", result);/
	// kconsole.listen();
	loop {}
}

