
# Rust学习笔记一:开发环境安装

# 目录
<!-- TOC -->

- [Rust学习笔记一:开发环境安装](#rust学习笔记一开发环境安装)
- [目录](#目录)
- [前言](#前言)
- [一、Rust介绍](#一rust介绍)
- [二、开发环境搭建步骤](#二开发环境搭建步骤)
  - [windows用户](#windows用户)
  - [在 Linux 或 macOS 上安装 rustup](#在-linux-或-macos-上安装-rustup)
    - [macOS 下](#macos-下)
    - [Linux 下](#linux-下)
  - [检查安装是否成功](#检查安装是否成功)
- [三、IDE配置](#三ide配置)
- [四、第一个Rust程序](#四第一个rust程序)
  - [创建第一个Rust项目](#创建第一个rust项目)
  - [运行项目](#运行项目)
    - [直接点击运行按钮运行](#直接点击运行按钮运行)
    - [使用cargo运行](#使用cargo运行)
    - [使用cargo编译后运行](#使用cargo编译后运行)
- [总结](#总结)

<!-- /TOC -->
# 前言

这是Rust系列的第一篇文章，本文将要带你认识Rust，并学习安装Rust开发环境，以及配置IDE，最后将完成一个`Hello,World`的代码示例。

# 一、Rust介绍

对于高级语言编程者如`Java、python、Go、C#`等，那些“系统层面”的工作涉及内存管理、数据表示和并发等底层细节，是一个陌生的领域，甚至在日常工作和学习中很少触及，只有在底层语言开发多年的人才可能最为了解，即使谨慎操作，也难免出现BUG。
Rust为底层开发做了很多优化，让底层开发更加安全的操作如内存管理、并发等，但是Rust并不局限于系统编程，它有着高级语言的表达力，性能高、线程安全等优势，因此Rust可以做的事情很多，比如编写Web应用的后端，[或是为你的树莓派编写操作系统](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials),亦或是写个[RPG游戏](https://github.com/veloren/veloren)。

# 二、开发环境搭建步骤

## windows用户

Windows 上安装 Rust 需要有 C++ 环境。
在 Windows 上，前往 <https://www.rust-lang.org/install.html> 并按照说明安装 Rust。在安装过程的某个步骤，你会收到一个信息说明为什么需要安装 Visual Studio 2013 或其更新版本的 MSVC 构建工具。

要获取构建工具，你需要安装 Visual Studio 2022。当被问及需要安装什么工作负载（Workload）的时候，请确保勾选了以下内容：

- “使用 C++ 的桌面开发”（“Desktop Development with C++”）
- Windows 10（或 11）SDK
- 英语语言包，以及其他你所需要的语言包
运行下载的安装程序 新手直接选1，后面默认安装即可。

```shell
PS C:\Users\Hehongyuan> rustup-init.exe
......
Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation

```

## 在 Linux 或 macOS 上安装 rustup

打开终端并输入下面命令：

```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

这个命令将下载一个脚本并开始安装 rustup 工具，此工具将安装 Rust 的最新稳定版本。可能会提示你输入管理员密码。

如果安装成功，将出现下面这行：

```shell
Rust is installed now. Great!
```

OK，这样就已经完成 Rust 安装啦。

Rust 对运行环境的依赖和 Go 语言很像，几乎所有环境都可以无需安装任何依赖直接运行。但是，Rust 会依赖 libc 和链接器 linker。所以如果遇到了提示链接器无法执行的错误，你需要再手动安装一个 C 语言编译器：

### macOS 下

```shell
xcode-select --install
```

### Linux 下

Linux 用户一般应按照相应发行版的文档来安装 GCC 或 Clang。

例如，如果你使用 Ubuntu，则可安装 build-essential。

## 检查安装是否成功

打开命令行：

```shell
$ rustc -V
rustc 1.56.1 (59eed8a2a 2021-11-01)

$ cargo -V
cargo 1.57.0 (b2e52d7ca 2021-10-21)

```

恭喜，你已成功安装 Rust！

# 三、IDE配置

这里推荐使用VSCode，如果你习惯使用Jetbrains 全家桶的可以下载[RustRover](https://www.jetbrains.com/rust/)
这里只介绍一下VSCode的方式，下载好后安装插件`rust-analyzer`，其他插件推荐：

 1. `Even Better TOML`，支持 .toml 文件完整特性
 2. `Error Lens`, 更好的获得错误展示
 3. `CodeLLDB`, Debugger 程序

# 四、第一个Rust程序

首先这里说明一下Rust的包管理器是`cargo`,这个后面会介绍，它就像前端的`npm`,python的`pip`。

## 创建第一个Rust项目

在`VSCode`选择终端->新建终端，在终端中输入：

```shell
cargo new hello_world
cd hello_world
```

这是我们得到一个创建好的项目文件，他的文件结构如下

```shell
.
├── .git
├── .gitignore
├── Cargo.toml
└── src
    └── main.rs
```

打开`main.rs`,里面已经默认写了一个hello,world程序

```rust
fn main() {
    println!("Hello, world!");
}
```

## 运行项目

### 直接点击运行按钮运行

在`vscode`中可以直接点击`main`函数上面的运行按钮运行

### 使用cargo运行

直接在终端运行命令：

```shell
cargo run
Compiling world_hello v0.1.0 (/Users/sunfei/development/rust/world_hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/world_hello`
Hello, world!
```

### 使用cargo编译后运行

编译：

```shell
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```

运行：

```shell
$ ./target/debug/world_hello
Hello, world!
```

`提示：Windows 得到的编译结果 应该是个.exe文件`

---

# 总结

以上就是今天要讲的内容，本文仅仅简单介绍了Rust 的准备环境搭建和开发环境的配置，最后介绍了Rust的运行方式。
