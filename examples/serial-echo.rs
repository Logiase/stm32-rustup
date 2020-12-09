#![no_main]
#![no_std]
#![allow(dead_code)]

// panic handler
extern crate panic_semihosting;

// attribute
use cortex_m_rt::entry;
use stm32f4xx_hal::interrupt;

// alias
use stm32f4xx_hal as hal;

// dependencies
use stm32f4xx_hal::prelude::*;

use hal::{
    stm32,
    rcc::{Rcc, Clocks},
    serial,
    gpio::{AF7, Alternate, gpioa},
};

// global
static mut SERIAL1: Option<serial::Serial<
    stm32::USART1,
    (gpioa::PA9<Alternate<AF7>>, gpioa::PA10<Alternate<AF7>>),
>> = None;
static mut SERIAL1_BUFFER: Buffer = Buffer::new();

// Serial Buffer
const RX_BUF_SIZE: usize = 512;

struct Buffer {
    index: usize,
    buffer: [u8; RX_BUF_SIZE],
}

impl Buffer {
    const fn new() -> Buffer {
        Buffer {
            index: 0,
            buffer: [0; RX_BUF_SIZE],
        }
    }

    fn push(&mut self, data: u8) -> Result<(), ()> {
        if self.index < RX_BUF_SIZE {
            self.buffer[self.index] = data;
            self.index += 1;
            return Ok(());
        }
        Err(())
    }

    fn read(&mut self) -> Option<Buffer> {
        if self.index > 0 {
            let tmp = self.index;
            self.index = 0;
            Some(Buffer {
                index: tmp,
                buffer: self.buffer,
            })
        } else {
            None
        }
    }
}

// entry
#[entry]
fn main() -> ! {
    // get peripherals
    let dp = stm32::Peripherals::take().unwrap();
    let _cp = stm32::CorePeripherals::take().unwrap();

    // init clock
    let rcc = dp.RCC.constrain();
    let clocks = setup_clocks(rcc);

    // gpio
    let ga = dp.GPIOA.split();

    // init USART1
    let mut serial1 = serial::Serial::usart1(
        dp.USART1,
        (ga.pa9.into_alternate_af7(), ga.pa10.into_alternate_af7()),
        serial::config::Config::default().baudrate(115_200.bps()),
        clocks,
    ).unwrap();
    serial1.listen(serial::Event::Rxne);
    unsafe { SERIAL1 = Some(serial1) }

    loop {}
}

#[interrupt]
fn USART1() {
    let serial1 = unsafe { SERIAL1.as_mut().unwrap() };
    if serial1.is_rxne() {
        let b = serial1.read().unwrap();
        if b != b'\n' {
            unsafe { SERIAL1_BUFFER.push(b).unwrap(); }
        } else {
            if serial1.read().unwrap() == b'\r' {
                unsafe {
                    for i in 0..SERIAL1_BUFFER.index {
                        serial1.write(SERIAL1_BUFFER.buffer[i]).unwrap();
                    }
                }
            }
        }
    }
}

fn setup_clocks(rcc: Rcc) -> Clocks {
    return rcc
        .cfgr
        .use_hse(25.mhz())
        .sysclk(180.mhz())
        .freeze();
}