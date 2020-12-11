#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};

extern crate panic_halt;
extern crate stm32f4xx_hal;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("ready");

    loop {}
}