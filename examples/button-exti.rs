#![no_main]
#![no_std]

extern crate panic_rtt_target;
extern crate stm32f4xx_hal as hal;

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use hal::{
    gpio::{gpiob, gpioc, gpioh, Edge, ExtiPin, Input, Output, PullUp, PushPull},
    prelude::*,
    stm32::{self, interrupt, Interrupt},
};
use rtt_target::{rprintln, rtt_init_print};

static LED_GREEN: Mutex<RefCell<Option<gpiob::PB0<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));
static LED_RED: Mutex<RefCell<Option<gpiob::PB1<Output<PushPull>>>>> =
    Mutex::new(RefCell::new(None));
static KEY0: Mutex<RefCell<Option<gpioh::PH3<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
static KEY1: Mutex<RefCell<Option<gpioh::PH2<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
static KEY2: Mutex<RefCell<Option<gpioc::PC13<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn start() -> ! {
    rtt_init_print!();
    let mut device = stm32::Peripherals::take().unwrap();
    let _core = stm32::CorePeripherals::take().unwrap();

    device.RCC.apb2enr.write(|w| w.syscfgen().enabled());

    let rcc = device.RCC.constrain();
    let _clocks = rcc.cfgr.use_hse(25.mhz()).sysclk(72.mhz()).freeze();

    let gb = device.GPIOB.split();
    let gc = device.GPIOC.split();
    let gh = device.GPIOH.split();

    let led_green = gb.pb0.into_push_pull_output();
    let led_red = gb.pb1.into_push_pull_output();

    let mut key0 = gh.ph3.into_pull_up_input();
    let mut key1 = gh.ph2.into_pull_up_input();
    let mut key2 = gc.pc13.into_pull_up_input();

    key0.make_interrupt_source(&mut device.SYSCFG);
    key0.enable_interrupt(&mut device.EXTI);
    key0.trigger_on_edge(&mut device.EXTI, Edge::FALLING);

    key1.make_interrupt_source(&mut device.SYSCFG);
    key1.enable_interrupt(&mut device.EXTI);
    key1.trigger_on_edge(&mut device.EXTI, Edge::FALLING);

    key2.make_interrupt_source(&mut device.SYSCFG);
    key2.enable_interrupt(&mut device.EXTI);
    key2.trigger_on_edge(&mut device.EXTI, Edge::FALLING);

    unsafe {
        stm32::NVIC::unmask(Interrupt::EXTI15_10);
        stm32::NVIC::unmask(Interrupt::EXTI2);
        stm32::NVIC::unmask(Interrupt::EXTI3);
    }

    free(|cs| {
        LED_GREEN.borrow(cs).replace(Some(led_green));
        LED_RED.borrow(cs).replace(Some(led_red));

        KEY0.borrow(cs).replace(key0.into());
        KEY1.borrow(cs).replace(key1.into());
        KEY2.borrow(cs).replace(key2.into());
    });

    rprintln!(
        "{}, {}, {}",
        stm32::NVIC::is_enabled(Interrupt::EXTI2),
        stm32::NVIC::is_enabled(Interrupt::EXTI3),
        stm32::NVIC::is_enabled(Interrupt::EXTI15_10)
    );

    loop {}
}

enum BtnPressed {
    Key0Pressed,
    Key1Pressed,
    Key2Pressed,
}

fn key_pressed(btn: BtnPressed) {
    free(|cs| {
        if let (Some(ref mut led_red), Some(ref mut led_green)) = (
            LED_RED.borrow(cs).borrow_mut().deref_mut(),
            LED_GREEN.borrow(cs).borrow_mut().deref_mut(),
        ) {
            match btn {
                BtnPressed::Key0Pressed => {
                    led_green.toggle().unwrap();
                }
                BtnPressed::Key1Pressed => {
                    led_green.set_high().unwrap();
                    led_red.set_high().unwrap();
                }
                BtnPressed::Key2Pressed => {
                    led_red.toggle().unwrap();
                }
            }
        }
    });
}

#[interrupt]
fn EXTI2() {
    rprintln!("EXTI2");
    free(|cs| {
        let mut btn_ref = KEY1.borrow(cs).borrow_mut();
        if let Some(ref mut btn) = btn_ref.deref_mut() {
            key_pressed(BtnPressed::Key1Pressed);
            btn.clear_interrupt_pending_bit();
        }
    });
}

#[interrupt]
fn EXTI3() {
    rprintln!("EXTI3");
    free(|cs| {
        let mut btn_ref = KEY0.borrow(cs).borrow_mut();
        if let Some(ref mut btn) = btn_ref.deref_mut() {
            key_pressed(BtnPressed::Key0Pressed);
            btn.clear_interrupt_pending_bit();
        }
    });
}

#[interrupt]
fn EXTI15_10() {
    rprintln!("EXTI15-10");
    free(|cs| {
        let mut btn_ref = KEY2.borrow(cs).borrow_mut();
        if let Some(ref mut btn) = btn_ref.deref_mut() {
            key_pressed(BtnPressed::Key2Pressed);
            btn.clear_interrupt_pending_bit();
        }
    });
}
