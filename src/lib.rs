#![no_std]

pub mod clock;

pub use clock::setup_clocks;


use core::panic::PanicInfo;
use rtt_target::rprintln;
#[inline(never)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {}
}