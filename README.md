# stm32-rustup

A guide to rust your stm32 microcontroller.

## my device

A stm32F429IGT6 MCU with some labels and a ST-Link debugger.

### used resource

The resource used in examples.

external high speed clock: 25MHz

- PB0 - A green LED
- PB1 - A red LED

## Transplant

1. first declaring your memory layout by editing `memory.x`
2. replace dependencies from `cargo.toml` to use your device-only HAL implementation
3. redeclare pins to feat your device or board

## Author

@Logiase

## LICENSE

[MIT LICENSE](./LICENSE)
