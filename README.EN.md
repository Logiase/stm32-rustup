# stm32-rustup

A guide about rustup your stm32 mcu.

WIP

## Why use Rust

Rust as a new-generation programming language has a lot of
modern features than C and C++.
Means that using Rust has a better experience.

And Rust has a great variety crates in `no_std`.

As Rust has a lot of modern features, it's possible to transplant your code
to other type of chips, even other target such as RISC-V and AVR.

See [Transplant](#Transplant)

![crates](/imgs/crates.png)

> well done !

## Try out

### IDE

I'm using VSCode, and install these extensions:

- crates
- rust-analyzer

> NOTE: bug fixed. Use latest version, at least `0.2.441`.
> there is a bug in the newest version of `rust-analyzer`,
> which can NOT parse macro properly, before the bug fixed,
> please use version `0.2.400` instead.
>
> newest version now: `0.2.408`

## device

my device is a STM32F429IGT6 chip and other peripherals
which I'll introduce in Section [resource](#resource)

As using Rust, you do not have to use a same board even a same target (ARM),
you can try the examples use your own device.

Oh, and a ST-Link debugger and TTL-Serial transformer and two usb cable

## Resource

The resource used in examples

High-Speed-External : 25MHz

- PB0 - A green LED
- PB1 - A red LED
- PA0 - WK_UP
- PC13 - key
- PH2 - key
- PH3 - key

## Transplant

If you want to try out the examples in your device

1. according to your device's manufacture, edit the memory layout `memory.x`
2. fix the dependencies in `cargo.toml`
3. redeclare pins in examples

And if you want try in another architecture chip

1. edit anything about target in `.cargo/config`
2. edit `cargo.toml` to add crates about HAL, PAC
3. Optionally, edit `.vscode/settings.json` to make vscode check your code correctly

## Read More

- [Lib.rs](https://lib.rs) you can find a lot `no_std' crates here
- [crates.io](https://crates.io)
- [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust)
  you can find a lot crates about embedded, such as hardware driver, HAL, PAC.
  
## Q&A

I'm also a noob.

So just open an issue, and I'll try my best to answer.

Give me a star to let me know this repo helps you and encourage me to do more.

## Author

@Logiase

## LICENSE

[MIT LICENSE](./LICENSE)
