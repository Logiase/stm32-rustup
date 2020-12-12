#![no_main]
#![no_std]

extern crate panic_rtt_target;

use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use hal::prelude::*;
use hal::{
    gpio::{gpioa, Alternate, AF7},
    interrupt, serial, stm32,
};
use rtt_target::{rprintln, rtt_init_print};
use stm32_rustup::buffer::BytesBuffer;
use stm32f4xx_hal as hal;

static SERIAL1: Mutex<
    RefCell<
        Option<
            serial::Serial<
                stm32::USART1,
                (gpioa::PA9<Alternate<AF7>>, gpioa::PA10<Alternate<AF7>>),
            >,
        >,
    >,
> = Mutex::new(RefCell::new(None));
static SERIAL1_BUF: Mutex<RefCell<Option<BytesBuffer>>> = Mutex::new(RefCell::new(None));

#[entry]
fn start() -> ! {
    rtt_init_print!();
    let device = stm32::Peripherals::take().unwrap();
    let _core = stm32::CorePeripherals::take().unwrap();
    let rcc = device.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(25.mhz()).sysclk(64.mhz()).freeze();

    let ga = device.GPIOA.split();

    let serial1 = serial::Serial::usart1(
        device.USART1,
        (ga.pa9.into_alternate_af7(), ga.pa10.into_alternate_af7()),
        serial::config::Config::default().baudrate(115_200.bps()),
        clocks,
    )
    .unwrap();

    free(|cs| {
        SERIAL1.borrow(cs).replace(Some(serial1));
        SERIAL1_BUF.borrow(cs).replace(Some(BytesBuffer::new()));
    });
    rprintln!("serial1 ok");

    loop {}
}

#[interrupt]
fn USART1() {
    free(|_cs| {

    });
}
