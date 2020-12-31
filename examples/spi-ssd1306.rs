#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use embedded_hal::spi::{Mode, Phase, Polarity};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use ssd1306::{prelude::*, Builder};
use stm32f4xx_hal::{delay::Delay, prelude::*, spi::Spi, stm32};

#[entry]
fn start() -> ! {
    rtt_init_print!();
    let core = stm32::CorePeripherals::take().unwrap();
    let device = stm32::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.mhz()).sysclk(48.mhz()).freeze();

    let gpioa = device.GPIOA.split();
    let gpioc = device.GPIOC.split();
    let gpioh = device.GPIOH.split();

    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();
    let dc = gpioc.pc5.into_push_pull_output();
    let cs = gpioh.ph13.into_push_pull_output();
    let mut rst = gpioc.pc4.into_push_pull_output();

    let mut delay = Delay::new(core.SYST, clocks);

    let spi1 = Spi::spi1(
        device.SPI1,
        (sck, miso, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        8_u32.mhz().into(),
        clocks,
    );

    let interface = SPIInterface::new(spi1, dc, cs);

    let mut disp: GraphicsMode<_,_> = Builder::new().connect(interface).into();
    disp.reset(&mut rst, &mut delay);
    disp.init().unwrap();
    disp.flush().unwrap();

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64, 64);
    let im = Image::new(&raw, Point::new(32, 0));

    im.draw(&mut disp).unwrap();
    disp.flush().unwrap();

    loop {}
}
