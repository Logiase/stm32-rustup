# stm32-rustup

English: [README.EN.md](README.EN.md)

Rust嵌入式开发指北。

## Why

我C、C++写的好好的，为什么要换Rust？

首先Rust作为一门新兴的语言，相比C有着更良好的使用体验，其次使用Rust可以享受到Rust丰富的`no_std`环境。

Rust具有的高级现代语言的特性使得其在不同型号芯片，甚至不同架构芯片上，仅仅通过简单的修改即可移植，See [移植](#移植)。

同时相比于`arm-gcc-none-eabi`，即标准的ARM嵌入式编译器，Rust有着更高的性能。

![crates](/imgs/crates.png)

> 什么叫舒服啊.jpg

## 尝试

目前有如下例子:

- hello 通过rtt输出 "Hello Embedded World"
- blink 闪烁LED
- button 读取GPIO控制LED
- usart-irq 通过串口中断读取USART, 实现复读机
- button-exti GPIO外部中断控制LED
- iwdg 独立看门狗
- timer 定时器控制LED
- pwm PWM控制LED
- rng 随机数发生器

### IDE

VSCode是一个很好的选择，你需要安装如下扩展：

- crates
- rust-analyzer

> 请注意：此bug已经修复, 请使用最新版本 `0.2.441`.
> 在最新版本的`rust-analyzer`中存在着无法解析宏的Bug, 在修复之前请使用 0.2.400 版本
> 当前最新版本：0.2.416

### 编译

使用如下命令进行编译 `blink`

```shell
cargo build --example blink
```

编译后的ELF产物在`target/thumbv7em-none-eabihf/debug/`目录下，执行`binutils`即可查看相关信息。

第一次编译的过程会很长，如果你修改了内存布局`memory.x`，再次执行`cargo build`并不会重新应用你的内存布局，
你需要清除掉之前的缓存重新进行编译才可以应用新的内存布局

```shell
cargo clean
cargo build --example some_bin
```

## 设备

stm32F429IGT6，若干导线，ST-Link调试器，TTL转串口。

### 资源

例子中用到的外设资源：

高速外部时钟源 25MHz

- PB0 - 绿色LED
- PB1 - 红色LED
- PA0 - WK_UP
- PC13 - 按键
- PH2 - 按键
- PH3 - 按键

在我的博客上有更多有关嵌入式Rust的相关内容，也在持续更新中。

-> [Blog](https://blog.logiase.site) <-

## 移植

如果想要在你的设备上进行实验，请注意一些内容：

1. 首先根据自己的设备修改内存布局`memory.x`
2. 从`cargo.toml`中替换HAL，如果你的设备不是ARM内核，请同时修改`cortex-m`至你的目标架构
3. 根据你的设备重新定义源码中的引脚

如果你要在不同目标平台上进行修改尝试，请执行以下操作：

1. 修改`.cargo/config`文件，将其中有关target的内容修改为你的目标架构
2. 修改`cargo.toml`，引入你设备的布局，HAL等
3. 可选，修改`.vscode/settings.json`中的内容来让VSCode正确检查你的代码

## 更多资源

- [Lib.rs](https://lib.rs)
  在这你可以找到更多`no_std`库
- [crates.io](https://crates.io)
  同上
- [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust)
  在这你可以找到更多有关Rust嵌入式开发的资料，如硬件API，HAL，PAC

## Q&A

Open an Issues And I‘ll try my best to answer.

## Author

@Logiase

## LICENSE

[MIT LICENSE](./LICENSE)
