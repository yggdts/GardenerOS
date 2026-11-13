#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_hanndler]
fn panic(_info: &PanicInfo) -> ! {
	loop{}
}
