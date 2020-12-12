#![no_main]
#![no_std]

use stm32f4xx_hal as hal;

use cortex_m_rt::entry;

use hal::prelude::*;
use stm32_rustup::*;

use crate::ButtonDown::{Key0Pressed, Key1Pressed, Key2Pressed, NoPressed, WkUpPressed};
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use hal::{
    delay,
    gpio::{gpioa, gpioc, gpioh, Input, PullDown, PullUp},
    stm32,
};
use rtt_target::{rprintln, rtt_init_print};
use switch_hal::{
    ActiveHigh, ActiveLow, InputSwitch, IntoSwitch, OutputSwitch, Switch, ToggleableOutputSwitch,
};

// global instance
static DELAY: Mutex<RefCell<Option<delay::Delay>>> = Mutex::new(RefCell::new(None));
static KEY0: Mutex<RefCell<Option<Switch<gpioh::PH3<Input<PullUp>>, ActiveLow>>>> =
    Mutex::new(RefCell::new(None));
static KEY1: Mutex<RefCell<Option<Switch<gpioh::PH2<Input<PullUp>>, ActiveLow>>>> =
    Mutex::new(RefCell::new(None));
static KEY2: Mutex<RefCell<Option<Switch<gpioc::PC13<Input<PullUp>>, ActiveLow>>>> =
    Mutex::new(RefCell::new(None));
static WK_UP: Mutex<RefCell<Option<Switch<gpioa::PA0<Input<PullDown>>, ActiveHigh>>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = stm32::Peripherals::take().unwrap();
    let cp = stm32::CorePeripherals::take().unwrap();

    let clocks = setup_clocks(dp.RCC.constrain());

    free(|cs| {
        DELAY
            .borrow(cs)
            .replace(Some(delay::Delay::new(cp.SYST, clocks)))
    });

    let ga = dp.GPIOA.split();
    let gb = dp.GPIOB.split();
    let gc = dp.GPIOC.split();
    let gh = dp.GPIOH.split();

    let mut led_green = gb.pb0.into_push_pull_output().into_active_low_switch();
    let mut led_red = gb.pb1.into_push_pull_output().into_active_low_switch();

    free(|cs| {
        KEY0.borrow(cs)
            .replace(Some(gh.ph3.into_pull_up_input().into_active_low_switch()));
        KEY1.borrow(cs)
            .replace(Some(gh.ph2.into_pull_up_input().into_active_low_switch()));
        KEY2.borrow(cs)
            .replace(Some(gc.pc13.into_pull_up_input().into_active_low_switch()));
        WK_UP.borrow(cs).replace(Some(
            ga.pa0.into_pull_down_input().into_active_high_switch(),
        ))
    });

    rprintln!("ready");

    loop {
        match read_button() {
            Key0Pressed => {
                led_green.toggle().unwrap();
            }
            Key1Pressed => {
                led_green.off().unwrap();
                led_red.off().unwrap();
            }
            Key2Pressed => {
                led_red.toggle().unwrap();
            }
            WkUpPressed => {
                led_red.on().unwrap();
                led_green.on().unwrap();
            }
            NoPressed => {}
        }
    }
}

enum ButtonDown {
    Key0Pressed,
    Key1Pressed,
    Key2Pressed,
    WkUpPressed,
    NoPressed,
}

/// check button down
/// KEY0 > KEY1 > KEY2 > WK_UP
fn read_button() -> ButtonDown {
    static mut KEY_UP: bool = true;

    // get current button status
    fn read_status() -> (bool, bool, bool, bool) {
        free(|cs| {
            let key0 = KEY0
                .borrow(cs)
                .borrow()
                .as_ref()
                .unwrap()
                .is_active()
                .unwrap();
            let key1 = KEY1
                .borrow(cs)
                .borrow()
                .as_ref()
                .unwrap()
                .is_active()
                .unwrap();
            let key2 = KEY2
                .borrow(cs)
                .borrow()
                .as_ref()
                .unwrap()
                .is_active()
                .unwrap();
            let wk_up = WK_UP
                .borrow(cs)
                .borrow()
                .as_ref()
                .unwrap()
                .is_active()
                .unwrap();
            return (key0, key1, key2, wk_up);
        })
    }

    free(|cs| {
        let status = read_status();
        unsafe {
            if KEY_UP && (status.0 || status.1 || status.2 || status.3) {
                rprintln!("{}, {}, {}, {}", status.0, status.1, status.2, status.3);
                if let Some(ref mut delay) = DELAY.borrow(cs).borrow_mut().deref_mut() {
                    delay.delay_ms(10_u8);
                }
                KEY_UP = false;
                let status = read_status();
                if status.0 {
                    return Key0Pressed;
                } else if status.1 {
                    return Key1Pressed;
                } else if status.2 {
                    return Key2Pressed;
                } else if status.3 {
                    return WkUpPressed;
                }
            } else if read_status().eq(&(false, false, false, false)) {
                KEY_UP = true;
            }
            return NoPressed;
        }
    })
}
