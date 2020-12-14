#![no_main]
#![no_std]

extern crate panic_rtt_target;

use cortex_m_rt::entry;
use hal::prelude::*;
use hal::{
    delay, pwm,
    stm32::{self},
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

    let mut delay = delay::Delay::new(core.SYST, clocks);

    let gpiob = device.GPIOB.split();
    let timer3_pwm_pins = (
        gpiob.pb4.into_alternate_af2(),
        gpiob.pb5.into_alternate_af2(),
        gpiob.pb0.into_alternate_af2(),
        gpiob.pb1.into_alternate_af2(),
    );

    let pwm_channels = pwm::tim3(device.TIM3, timer3_pwm_pins, clocks, 20.mhz());
    let mut pb1 = pwm_channels.3;
    let max_duty = pb1.get_max_duty();

    rprintln!("loop start");
    loop {
        for i in 1..16 {
            rprintln!("{}", i);
            pb1.disable();
            pb1.set_duty(max_duty / i);
            pb1.enable();
            delay.delay_ms(3000_u32);
        }
    }
}
