use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::rcc;

/// 设置时钟
/// `use_hse` 使用外部高速晶振
/// `sysclk` 系统频率
/// and ...
pub fn setup_clocks(r: rcc::Rcc) -> rcc::Clocks {
    return r
        .cfgr
        .use_hse(25.mhz())
        .sysclk(180.mhz())
        .freeze();
}