#![no_main]
#![no_std]

// alias
use stm32f4xx_hal as hal;

// attribute
use cortex_m_rt::entry;

// extern use
extern crate panic_halt;

// prelude
use hal::prelude::*;

// import
use hal::{
    stm32,
    rcc,
    delay,
};
use rtt_target::{rtt_init_print, rprintln};

// start
#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = stm32::Peripherals::take().unwrap();
    let cp = stm32::CorePeripherals::take().unwrap();

    // setup clocks
    let clocks = setup_clocks(dp.RCC.constrain());

    // delay
    let mut delays = delay::Delay::new(cp.SYST, clocks);

    // gpio
    let gb = dp.GPIOB.split();

    // led
    let mut led_green = gb.pb0.into_push_pull_output();
    let mut led_red = gb.pb1.into_push_pull_output();

    rprintln!("ready");

    loop {
        rprintln!("new loop");
        led_green.toggle().unwrap();
        delays.delay_ms(500_u32);
        led_red.toggle().unwrap();
        delays.delay_ms(500_u32);
    }
}

fn setup_clocks(r: rcc::Rcc) -> rcc::Clocks {
    return r
        .cfgr
        .use_hse(25.mhz())
        .sysclk(180.mhz())
        .freeze();
}