# Rust学习笔记二:认识cargo

## 目录
<!-- TOC -->

- [Rust学习笔记二:认识cargo](#rust学习笔记二认识cargo)
  - [目录](#目录)
  - [前言](#前言)
  - [介绍Cargo](#介绍cargo)
  - [Cargo项目](#cargo项目)
    - [Cargo.toml文件](#cargotoml文件)
    - [main.rs文件](#mainrs文件)
  - [构建并运行 Cargo 项目](#构建并运行-cargo-项目)
  - [发布（release）构建](#发布release构建)
  - [总结](#总结)

<!-- /TOC -->
## 前言

第一章的最后我们使用`cargo`创建了`hello_world`项目，本章详细介绍一下cargo

---

## 介绍Cargo

Cargo 是 Rust 的构建系统和包管理器。它可以为你处理很多任务，比如构建代码、下载依赖库并编译这些库。
我们在第一章的`hello_world`项目已经使用了Cargo。

## Cargo项目

我们回看 `hello_world`项目，会看到 Cargo 生成了两个文件和一个目录：一个 Cargo.toml 文件，一个 src 目录，以及位于 src 目录中的 main.rs 文件。目录初始化了一个 git 仓库，以及一个 .gitignore 文件。如果在一个已经存在的 git 仓库中运行 cargo new，则这些 git 相关文件则不会生成；可以通过运行 cargo new --vcs=git 来覆盖这些行为。

### Cargo.toml文件

使用`vscode`打开`Cargo.toml`

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```

这个文件使用 TOML (Tom's Obvious, Minimal Language) 格式，这是 Cargo 配置文件的格式。

第一行，`[package]`，是一个片段（section）标题，表明下面的语句用来配置一个包。随着我们在这个文件增加更多的信息，还将增加其他片段（section）。

接下来的三行设置了 Cargo 编译程序所需的配置：项目的名称、项目的版本以及要使用的 Rust 版本。 `edition` 字段定义了我们使用的 Rust 大版本。

最后一行，`[dependencies]`，是罗列项目依赖的片段的开始。在 Rust 中，代码包被称为 crates。这个项目并不需要其他的 crate，不过在第二章的第一个项目会用到依赖，那时会用得上这个片段。

### main.rs文件

现在打开 src/main.rs 看看：

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo 为你生成了一个 “Hello, world!” 程序。

Cargo 期望源文件存放在 src 目录中。项目根目录只存放 README、license 信息、配置文件和其他跟代码无关的文件。使用 Cargo 帮助你保持项目干净整洁，一切井井有条。

如果没有使用 Cargo 开始项目，比如我们创建的 Hello,world! 项目，可以将其转化为一个 Cargo 项目。将代码放入 src 目录，并创建一个合适的 Cargo.toml 文件。

## 构建并运行 Cargo 项目

cargo 编译命令：

```shell
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs

```

这个命令会创建一个可执行文件 target/debug/hello_world （在 Windows 上是 target\debug\hello_world.exe），而不是放在目前目录下。由于默认的构建方法是调试构建（debug build），Cargo 会将可执行文件放在名为 debug 的目录中。可以通过这个命令运行可执行文件：

```shell
$ ./target/debug/hello_world # 或者在 Windows 下为 .\target\debug\hello_world.exe
Hello, world!

```

首次运行 cargo build 时，也会使 Cargo 在项目根目录创建一个新文件：Cargo.lock。这个文件记录项目依赖的实际版本。这个项目并没有依赖，所以其内容比较少。你自己永远也不需要碰这个文件，让 Cargo 处理它就行了。

可以使用 cargo run 在一个命令中同时编译并运行生成的可执行文件：

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!

```

使用 cargo run 可以实现完全相同的效果，而且要方便得多，所以大多数开发者会使用 cargo run。

`注意这一次并没有出现表明 Cargo 正在编译 hello_world 的输出。Cargo 发现文件并没有被改变，所以它并没有重新编译，而是直接运行了可执行文件。如果修改了源文件的话，Cargo 会在运行之前重新构建项目。`

Cargo 还提供了一个叫 cargo check 的命令。该命令快速检查代码确保其可以编译，但并不产生可执行文件：

```shell
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs

```

通常 cargo check 要比 cargo build 快得多，如果你在编写代码时持续的进行检查，`cargo check` 可以让你快速了解现在的代码能不能正常通过编译！

## 发布（release）构建

当项目最终准备好发布时，可以使用 cargo build --release 来优化编译项目。这会在 target/release 而不是 target/debug 下生成可执行文件。这些优化可以让 Rust 代码运行的更快，不过启用这些优化也需要消耗更长的编译时间。这也就是为什么会有两种不同的配置：一种是为了开发，你需要经常快速重新构建；另一种是为用户构建最终程序，它们不会经常重新构建，并且希望程序运行得越快越好。如果你在测试代码的运行时间，请确保运行 cargo build --release 并使用 target/release 下的可执行文件进行测试。

## 总结

我们回顾下已学习的 Cargo 内容：

- 可以使用 `cargo new` 创建项目。
- 可以使用`cargo build` 构建项目。
- 可以使用`cargo run` 一步构建并运行项目。
- 可以使用 `cargo check` 在不生成二进制文件的情况下构建项目来检查错误。
- 有别于将构建结果放在与源码相同的目录，Cargo 会将其放到 target/debug 目录。

使用 Cargo 的一个额外的优点是，不管你使用什么操作系统，其命令都是一样的。所以从现在开始本书将不再为 Linux 和 macOS 以及 Windows 提供相应的命令。
