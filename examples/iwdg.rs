#![no_main]
#![no_std]

extern crate panic_rtt_target;

use cortex_m_rt::entry;
use hal::prelude::*;
use hal::{
    delay,
    stm32::{self},
    time::MilliSeconds,
    watchdog::IndependentWatchdog,
};
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

#[entry]
fn start() -> ! {
    rtt_init_print!();

    let device = stm32::Peripherals::take().unwrap();
    let core = stm32::CorePeripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.mhz()).sysclk(180.mhz()).freeze();

    let gb = device.GPIOB.split();
    let gh = device.GPIOH.split();

    let mut led_green = gb.pb0.into_push_pull_output();
    let mut key = gh.ph3.into_pull_up_input();
    let mut delays = delay::Delay::new(core.SYST, clocks);

    let mut watchdog = IndependentWatchdog::new(device.IWDG);

    delays.delay_ms(3000_u32);

    rprintln!("start");
    led_green.set_low().unwrap();

    watchdog.start(MilliSeconds(3000));

    loop {
        if key.is_low().unwrap() {
            watchdog.feed();
            rprintln!("feed");
        }
        delays.delay_ms(50_u32);
    }
}
