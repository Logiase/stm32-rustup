#![no_main]
#![no_std]

use stm32f4xx_hal as hal;

use cortex_m_rt::entry;

extern crate panic_halt;

use hal::prelude::*;
use stm32_rustup::*;

use hal::{
    stm32,
    delay,
};
use switch_hal::{InputSwitch, Switch, ActiveLow, IntoSwitch, OutputSwitch, ToggleableOutputSwitch, ActiveHigh};
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = stm32::Peripherals::take().unwrap();
    let cp = stm32::CorePeripherals::take().unwrap();

    let clocks = setup_clocks(dp.RCC.constrain());

    let mut delays = delay::Delay::new(cp.SYST, clocks);

    let ga = dp.GPIOA.split();
    let gb = dp.GPIOB.split();
    let gc = dp.GPIOC.split();
    let gh = dp.GPIOH.split();

    let mut led_green = gb.pb0.into_push_pull_output().into_active_low_switch();
    let mut led_red = gb.pb1.into_push_pull_output().into_active_low_switch();

    let mut key0 = gh.ph3.into_pull_up_input().into_active_low_switch();
    let mut key1 = gh.ph2.into_pull_up_input().into_active_low_switch();
    let mut key2 = gc.pc13.into_pull_up_input().into_active_low_switch();
    let mut wk_up = ga.pa0.into_pull_down_input().into_active_high_switch();

    rprintln!("ready");

    let mut key_up = true;

    loop {
        if key_up && (key0.is_active().unwrap() || key1.is_active().unwrap() || key2.is_active().unwrap() || wk_up.is_active().unwrap()) {
            rprintln!("{},{},{},{}",key0.is_active().unwrap(),key1.is_active().unwrap(),key2.is_active().unwrap(),wk_up.is_active().unwrap());
            delays.delay_ms(10_u8);
            key_up = false;
            if key0.is_active().unwrap() {
                led_green.toggle().unwrap();
            } else if key1.is_active().unwrap() {
                led_green.off().unwrap();
                led_red.off().unwrap();
            } else if key2.is_active().unwrap() {
                led_red.toggle().unwrap();
            } else if wk_up.is_active().unwrap() {
                led_red.on().unwrap();
                led_green.on().unwrap();
            }
        } else if !key0.is_active().unwrap() && !key1.is_active().unwrap() && !key2.is_active().unwrap() && !wk_up.is_active().unwrap() {
            key_up = true;
        }
    }
}

