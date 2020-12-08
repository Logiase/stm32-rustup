# Setup Rust Environment

如何在Windows上建立Rust开发环境

## Pre-Install

Rust依赖于MSVC或MinGW作为linker。 所以在安装Rust之前我们需要搭建一个MSVC或MinGW环境。

如果你已经有此环境，则可以跳过本节

### MSVC

MSVC是Visual Studio的编译工具。

配置MSVC环境最好的办法就是安装[Visual Studio 2019](https://www.visualstudio.com/) 。

如果你认为Visual Studio过于庞大，则可以仅安装Visual
Studio的编译工具[Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019) 。

使用MSVC作为linker，可以在Windows系统上获得更快的编译速度与更小的产物大小。

### MinGW

MinGW是针对Windows的GNU编译工具。

## Install

使用官方提供的安装工具安装

[rustup-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)

在安装过程中针对已安装的linker进行更改。

> 如果碰到网络问题，可以使用各个开源镜像站提供的镜像
> 使用如下指令设置临时环境变量
> ```powershell
> # 临时环境变量
> $env:RUSTUP_DIST_SERVER="https://mirrors.tuna.tsinghua.edu.cn/rustup"
> 
> # 执行 `rustup-init.exe`
> .\rustup-init.exe
> ```

### check

使用如下指令验证是否安装成功：

``` powershell 
rustup -v
```

``` text
rustup 1.23.1 (3df2264a9 2020-11-30)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.48.0 (7eac88abb 2020-11-16)`
```

## IDE

> 工欲善其事，必先利其器

### Clion

基于Intellj平台的Clion可以完美支持Rust的开发

### VSCode



## Refers

- [Rust Programming Language](https://www.rust-lang.org/)
- [Visual Studio](https://www.visualstudio.com/)