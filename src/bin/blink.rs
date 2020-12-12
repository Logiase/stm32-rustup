#![no_main]
#![no_std]

// alias
use stm32f4xx_hal as hal;

// attribute
use cortex_m_rt::entry;

// prelude
use hal::prelude::*;
use stm32_rustup::*;

// import
use hal::{delay, stm32};
use rtt_target::{rprintln, rtt_init_print};

// program entry point
#[entry]
fn main() -> ! {
    rtt_init_print!();

    // 获取外设
    // get Peripherals
    let dp = stm32::Peripherals::take().unwrap();
    let cp = stm32::CorePeripherals::take().unwrap();

    // 设置时钟
    // set up clocks
    let clocks = setup_clocks(dp.RCC.constrain());

    // 初始化延时 (时钟实现)
    // get a delay
    let mut delays = delay::Delay::new(cp.SYST, clocks);

    // GPIOB
    let gb = dp.GPIOB.split();

    // led 推挽输出
    let mut led_green = gb.pb0.into_push_pull_output();
    let mut led_red = gb.pb1.into_push_pull_output();

    rprintln!("ready");

    loop {
        rprintln!("new loop");
        // 反转 500ms
        led_green.toggle().unwrap();
        delays.delay_ms(500_u32);
        led_red.toggle().unwrap();
        delays.delay_ms(500_u32);
    }
}
