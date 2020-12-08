#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_rt::pre_init;

use stm32f4xx_hal::prelude::*;

use stm32f4xx_hal::{
    stm32::{self, NVIC},
    rcc::RccExt,
    serial::{self},
    gpio::{gpioa, AF7},
    interrupt,
};
use stm32f4xx_hal::gpio::Alternate;

// global ref
static mut SERIAL1: Option<serial::Serial<stm32::USART1, (
    gpioa::PA9<Alternate<AF7>>,
    gpioa::PA10<Alternate<AF7>>
)>> = None;

#[pre_init]
unsafe fn init() {}

#[entry]
fn start() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = stm32::CorePeripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.mhz()).sysclk(180.mhz()).freeze();

    let ga = dp.GPIOA.split();
    let mut serial1 = serial::Serial::usart1(
        dp.USART1,
        (ga.pa9.into_alternate_af7(), ga.pa10.into_alternate_af7()),
        serial::config::Config::default().baudrate(115200.bps()),
        clocks,
    ).unwrap();
    serial1.listen(serial::Event::Rxne);

    unsafe {
        SERIAL1.replace(serial1);
    }

    NVIC::pend(stm32::interrupt::USART1);

    loop {}
}

#[interrupt]
fn USART1() {
    let serial1 = unsafe {
        SERIAL1.as_mut().unwrap()
    };

    let mut b = 0;

    if serial1.is_rxne() {
        b = serial1.read().unwrap();
        serial1.write(b).unwrap();
    }
}
