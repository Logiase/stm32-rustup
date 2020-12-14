#![no_main]
#![no_std]

extern crate panic_rtt_target;

use core::cell::{Cell, RefCell};
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use hal::{
    prelude::*,
    stm32::{self, interrupt, Interrupt},
    timer::{Event, Timer},
};
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

static TIMER3: Mutex<RefCell<Option<Timer<stm32::TIM3>>>> = Mutex::new(RefCell::new(None));
static LED_STATE: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[entry]
fn start() -> ! {
    rtt_init_print!();

    let device = stm32::Peripherals::take().unwrap();

    let mut _rcc = device.RCC.constrain();
    let clocks = _rcc.cfgr.use_hse(25.mhz()).sysclk(72.mhz()).freeze();

    let gb = device.GPIOB.split();

    let mut led_green = gb.pb0.into_push_pull_output();

    let mut timer = Timer::tim3(device.TIM3, 5.hz(), clocks);
    timer.listen(Event::TimeOut);
    free(|cs| {
        TIMER3.borrow(cs).replace(timer.into());
    });

    unsafe {
        stm32::NVIC::unmask(Interrupt::TIM3);
    }

    rprintln!("start loop");
    loop {
        if free(|cs| LED_STATE.borrow(cs).get()) {
            led_green.set_low().unwrap();
        } else {
            led_green.set_high().unwrap();
        }
    }
}

#[interrupt]
fn TIM3() {
    free(|cs| {
        let mut timer_ref = TIMER3.borrow(cs).borrow_mut();
        if let Some(ref mut timer) = timer_ref.deref_mut() {
            timer.clear_interrupt(Event::TimeOut);
        }
        let stats = LED_STATE.borrow(cs);
        stats.set(!stats.get());
    });
}
