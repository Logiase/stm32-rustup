#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};

// 引入未使用依赖以允许编译
// panic handler
extern crate panic_halt;
// 内存布局
extern crate stm32f4xx_hal;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("hello, embedded world!!!!!!!!!");

    loop {}
}