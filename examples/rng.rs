#![no_std]
#![no_main]

extern crate panic_rtt_target;

use cortex_m_rt::entry;
use hal::prelude::*;
use hal::{delay, stm32};
use rand_core::RngCore;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

#[entry]
fn start() -> ! {
    rtt_init_print!();

    let device = stm32::Peripherals::take().unwrap();
    let core = stm32::CorePeripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.mhz()).sysclk(72.mhz()).freeze();

    let mut delays = delay::Delay::new(core.SYST, clocks);

    let mut rand_source = device.RNG.constrain(clocks);

    rprintln!("start loop");
    loop {
        let rn = rand_source.next_u32();
        rprintln!("{}", rn);
        delays.delay_ms(3000_u32);
    }
}
