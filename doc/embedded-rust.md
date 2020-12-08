# Embedded Rust

## Add Rust Toolchains

Cortex-M0, M0+, M1(ARMv6-M):

```shell
rustup target add thumbv6m-none-eabi
```

Cortex-M3 (ARMv7-M):

```shell
rustup target add thumbv7m-none-eabi
```

Cortex-M4, M7 without hardware floating point (ARMv7E-M):

```shell
rustup target add thumbv7em-none-eabi
```

Cortex-M4F, M7F with hardware floating point (ARMv7E-M):

```shell
rustup target add thumbv7em-none-eabihf
```

> `none` 无系统
> 
> `eabi` Embedded ABI

## Tools

## cargo-embed

```shell
cargo install cargo-embed
```