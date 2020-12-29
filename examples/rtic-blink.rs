#![no_main]
#![no_std]
#![deny(warnings)]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{delay::Delay, prelude::*};
use stm32f4xx_hal::{
    gpio::{
        gpiob::{PB0, PB1},
        Output, PushPull,
    },
    interrupt, stm32,
};

type LedGreen = PB0<Output<PushPull>>;
type LedRed = PB1<Output<PushPull>>;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        led_green: LedGreen,
        led_red: LedRed,
        delays: Delay,
    }

    #[init(spawn = [t1])]
    fn init(cx: init::Context) -> init::LateResources {
        rtt_init_print!();

        let core: cortex_m::Peripherals = cx.core;
        let device: stm32::Peripherals = cx.device;

        let clocks = device.RCC.constrain().cfgr.use_hse(25.mhz()).freeze();
        let delays = Delay::new(core.SYST, clocks);

        let gb = device.GPIOB.split();

        let led_green = gb.pb0.into_push_pull_output();
        let led_red = gb.pb1.into_push_pull_output();

        cx.spawn.t1().unwrap();

        rprintln!("init return");

        init::LateResources {
            led_green,
            led_red,
            delays,
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        rprintln!("idle");
        rtic::pend(interrupt::USART1);
        loop {}
    }

    #[task(resources = [led_green, delays], spawn = [t2])]
    fn t1(cx: t1::Context) {
        rprintln!("t1");
        cx.resources.led_green.toggle().unwrap();
        cx.resources.delays.delay_ms(500_u32);
        cx.spawn.t2().unwrap();
        rprintln!("t1 stop");
    }

    #[task(resources = [led_red, delays], spawn = [t1])]
    fn t2(cx: t2::Context) {
        rprintln!("t2");
        cx.resources.led_red.toggle().unwrap();
        cx.resources.delays.delay_ms(500_u32);
        cx.spawn.t1().unwrap();
        rprintln!("t2 stop");
    }

    extern "C" {
        fn USART1();
        fn USART2();
    }
};
