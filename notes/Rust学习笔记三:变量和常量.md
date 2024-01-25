# Rust学习笔记三:变量和常量

## 前言

如果你学习过其他编程语言，那么你可能已经熟悉变量了。变量是存储在内存中的值的引用，可以更改值。变量可以是任何类型，包括结构体、枚举、函数、字符串、数组、元组、数字等等。

在其它大多数语言中，要么只支持声明可变的变量，要么只支持声明不可变的变量( 例如函数式语言 )，前者为编程提供了灵活性，后者为编程提供了安全性，而 Rust 选择了两者都要，既要灵活性又要安全性。

## 变量命名以及Rust变量的特征

在其他语言中，我们用 `var a = "Hello,world!"`的方式给`a`赋值，在Rust中，我们用`let a = "Hello,world!"`的方式给`a`赋值。

在Rust中变量默认是不可变的，也就是说，一旦你定义了一个变量，那么它的值就不能被改变。，当我们需要变量变为可变的，我们可以使用`let mut a = "Hello,world!"`的方式定义一个可变的变量。也就是在 `let` 后面加上 `mut` 。

如果一个变量不可变，那么它的值在定义后就不能被改变。可以查看`demo_02`项目文件中的代码。

```rust
fn main() {
    let a = "Hello,world!";
    println!("a:{}",a);
    a = "Hello,Rust";
    println!("a:{}",a);
}
```

保存文件，运行`cargo run`

```bash
cargo run
   Compiling demo_02 v0.1.0 (/Users/xxxxx/Coding/GithubProjects/Rust_Learning/projects/demo_02)
error[E0384]: cannot assign twice to immutable variable `a`
 --> src/main.rs:5:9
  |
3 |         let a = "Hello,world!";
  |             -
  |             |
  |             first assignment to `a`
  |             help: consider making this binding mutable: `mut a`
4 |         println!("a:{}",a);
5 |         a = "Hello,Rust";
  |         ^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `demo_02` (bin "demo_02") due to previous error
```

准确的错误信息是：cannot assign twice to immutable variable `a`，即变量`a`不能被赋值两次。
这种规则让我们的代码变得非常清晰，也给别人阅读代码带来便利。
但是如果确实需要修改变量的值，那么我们可以使用`mut`关键字，让变量变为可变的。看下面的例子：

```rust
fn main() {
    let mut a = "Hello,world!";
    println!("a:{}",a);
    a = "Hello,Rust";
    println!("a:{}",a);
}
```

运行`cargo run`，可以看到输出：

```bash
cargo run
   Compiling demo_02 v0.1.0 (/Users/xxxxxxx/Coding/GithubProjects/Rust_Learning/projects/demo_02)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/demo_02`
a:Hello,world!
a:Hello,Rust
```

## 使用下划线开头忽略未使用的变量

在Rust中，我们可以使用下划线作为变量的前缀，来忽略未使用的变量。

```rust
fn main() {
    let _b = "Hello,world!";
    let c = "Hello,Rust";
}
```

运行`cargo run`，可以看到输出：

```bash
warning: unused variable: `c`
  --> src/main.rs:23:9
   |
23 |     let c = "Hello,Rust";
   |         ^ help: if this is intentional, prefix it with an underscore: `_c`
   |
   = note: `#[warn(unused_variables)]` on by default
```

两个变量都是只有声明，没有使用，但是编译器却独独给出了 c 未被使用的警告，充分说明了 _ 变量名前缀在这里发挥的作用。

## 变量解构

在Rust中，我们可以使用变量解构来获取变量的值。let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：从一个相对复杂的变量中，匹配出该变量的一部分内容：

```rust
fn main() {
    let (a, mut b):(bool,bool) = (true,false);
    // a = true 不可变， b=false 可变
    println!("a:{},b:{}",a,b);
    b = true;
    assert_eq!(a,b);
}
```

运行`cargo run`，输出如下：

```bash
 cargo run
   Compiling demo_02 v0.1.0 (/Users/xxxxx/Coding/GithubProjects/Rust_Learning/projects/demo_02)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/demo_02`
a:true,b:false
```

## 常量

Rust变量的默认不可变很像其他语言的常量概念，与不可变变量一样，常量也是绑定到一个常量名且不允许更改的值，但是常量和变量之间存在一些差异：

- 常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
- 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注类型。

下面是常量的例子：

```rust
fn main() {
    const PI: f32 = 3.14;
    println!("PI_VALUE:{}",PI);
}
```

`Rust 常量的命名约定是全部字母都使用大写，并使用下划线分隔单词，另外对数字字面量可插入下划线以提高可读性）`

## 变量的覆盖(shadowing)

Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的，如下所示：

```rust
fn main() {
  let sd = 1;
  // 在main函数的作用域内对之前的x进行遮蔽
  let sd = sd+1;
  {
    // 在当前的花括号作用域内，对之前的x进行遮蔽
    let sd = sd * 5;
    println!("sd one:{}",sd);
  }
  println!("sd two:{}",sd);
}
```

运行`cargo run`，输出如下：

```bash
cargo run
   Compiling demo_02 v0.1.0 (/Users/xxxxx/Coding/GithubProjects/Rust_Learning/projects/demo_02)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/demo_02`
sd one:10
sd two:2
```

这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。甚至可以赋值不同类型值。

`本项目代码已上传到Github，地址：https://github.com/saltStarbucksCoffee/Rust_Learning`