# stm32-rustup

Rust嵌入式开发指北。

## 设备

stm32F429IGT6，若干导线，ST-Link调试器，TTL转串口。

### 资源

例子中用到的外设资源：

高速外部时钟源 25MHz

- PB0 - 绿色LED
- PB1 - 红色LED

在我的博客上有更多有关嵌入式Rust的相关内容，也在持续更新中。

-> [Blog](https://blog.logiase.site) <-

## 移植

如果想要在你的设备上进行实验，请注意一些内容：

1. 首先根据自己的设备修改内存布局`memory.x`
2. 从`cargo.toml`中替换HAL，如果你的设备不是ARM内核，请同时修改`cortex-m`至你的目标架构
3. 根据你的设备重新定义源码中的引脚

## 更多资源

- [Lib.rs](https://lib.rs)
  在这你可以找到更多`no_std`库
- [crates.io](https://crates.io)
  同上
- [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust)
  在这你可以找到更多有关Rust嵌入式开发的资料，如硬件API，HAL，PAC

## Author

@Logiase

## LICENSE

[MIT LICENSE](./LICENSE)
