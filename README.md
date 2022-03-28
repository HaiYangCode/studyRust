# Rust基础及区别于其他语言的特点

## Mac环境下安装及更新，卸载

```shell
#mac 环境下安装rust环境
curl https://sh.rustup.rs -sSf | sh #下载rust安装脚本
rustup update #更新rust环境
rustup self uninstall #卸载rust环境
```

## 项目结构介绍

### cargo介绍

#### 1. 文件夹目录结构

#### 2. Cargo.toml作用及使用

#### 3. Cargo.lock作用及使用

该文件是自动生成，记录项目中依赖的三方库信息，名字，版本

### cargo相关命令

```shell
cargo --version # 查看版本号
cargo build #编译项目，在根目录下的target/debug/项目名；找到执行文件。
cargo build --release #构建优化模式下的程序
cargo run #直接运行项目
cargo run --release #运行优化模式下运行程序
cargo check #校验代码的正确性
cargo new projectName #创建一个工程名为projectName
```

## 基本语法特点

## 函数定义

### 1. main的定义及特点

### 1. 定义一个函数使用`fn`关键字

fn 定义一个函数

```rust
fn main() {
    println!("hello world");
}
```

### 2. 函数中知识点

1. Rust风格使用4个空格键而不是tab实现缩进。
2. 调用了一个叫`println！`的宏，如果是普通函数需要去掉`!`号。

* 宏调用和普通函数调用有什么区别？

4. 使用`;`作为一行代码的结尾

### 2. 有参数怎么获取？

`main`函数默认没有参数,如果有参数的话需要将他们放置在括号`()`中。

### rustfmt的作用及使用

用于格式化rust代码。

### 在命令行中运行一个rust程序

```shell
rustc main.rs # 运行简单的rust程序
```

### `crate`和`package`的区别

`crate`是Rust中的最小的编译单元，`package`是单个或多个crate的集合，因为单个`crate`也是一个`package`， 但`package`通常倾向于多个crate的组合，本书中`crate`和`package`
统一被翻译为包，只在两者同事出现且需要区别对待时， 将`crate`译为单元包，将`package`译为包。

### use 作用

导入非预导入的标准库

### 如何导入第三方库？

在`Cargo.toml`中的dependencies后面添加如下

```shell
rand = "0.8.5" # 库名+版本号
```

### 用match表达式替换expect

```rust

// let guess: u32 = guess.trim().parse().expect("please type a number");
let guess: u32 = match guess.trim().parse()//这里的match很关键
{
Ok(num) => num,
Err(_) => continue,
};
```

### 关联函数

### main函数所在的文件名有没有要求？

### 一共工程中能有几个main函数？

#### 常量的定义，

```rust
const MAX_POINTS: u32 = 100000;
```

1. let 和const修饰的常量有什么不同？
2. 定义常量是必须定义其类型。
3. 常量可以声明在任何地方，包括全局作用域。
4. 常量只能绑定到一个常量表达式上，而不能绑定在一个函数上。

#### 隐藏

在一个作用域中，可以声明相同的变量，新声明的变量可以覆盖之前声明的变量。该情况在rust中叫做隐藏。描述为：第一个变量被第二个变量隐藏了。

```rust
let x = 5;
let x = x + 8;
let x = x * 2;
println!("the value is :{}", x);
```

这个隐藏的功能和`let mut x = 3;x = x+3;x=x * 2`有什么区别？

#### 使用隐藏可以保证值在改变后，该变量是不可改变类型。

#### let是定义变量，可以在隐藏的过程中改变该变量的类型。

## 标量类型和符合类型

### 标量类型

i32和u32中，i表示有符号，u表示正数，无符号。 isize和usize根据不同的目标平台，在64位的架构是就是64位，在32位上就是32位。 可以使用_(分隔符)如：1_000.

#### 自动类型推导，在某些时候需要手动的指明类型。

#### 整数溢出

1. 在整数溢出时，debug模式下回报错。在release模式下会出发环绕模式。

#### 浮点类型

1. 默认浮点类型是f64
2. 浮点类型分为：f32 和 f64

#### 3. 浮点类型是否存在精度丢失或错乱的问题？

#### bool 类型

```rust
let t = false;
```

#### 字符类型

```rust
let c = 'C';

```

### 复合类型

复合类型包含：元组和数组（跟go相似）

#### 元组

```rust
fn main() {
    let up: (i32, i64, u8) = (500, 6.4, 1);
    //解构元组,这是个很重要的概念
    let (x, y, z) = up;
    println!("y value is :{}", y);
}

```

##### 获取元组中的值

1. 解构 ，将一个元组解构成可以获取的值。` let (x, y, z) = up;`
2. `up.0;`或则`up.1;`，元组是从0开始的

#### 数组类型

1. 数组在栈上创建
2. 定义后大小不能改变
3. 数组中装的都是同一个类型的值
4. 使用`vector`类型创建一个动态数组
5. 创建数组的方式及初始化值

```rust
fn main() {
    let a = [3; 5];
    //等价与
    let b = [3, 3, 3, 3, 3];
}
```

6. 如果访问数组，下标越界将会触发panic。

#### 函数

1. 使用`fn`定义函数
2. 在函数签名中，你必须显示的声明每个参数的类型

``python中不需要显示的指定参数的类型``

##### 3. 函数的返回值

```rust
fn five() -> u32 {
    5
}

```

### rust是一门基于表达式的语言

rust将statement和`expression`定义为了两个概念

#### 语句：那些执行操作但不返回值的操作

```rust
fn main() {
    let x = (
    let y = 3);//这是错误的，因为let x = 3;是一条语句而不是表达式。
}
```

#### 表达式：会进行计算并产生一个值作为结果的指令

1. 宏，调用函数都是表达式，上述中的3也是一个表达式。返回本身。
2. 创建一个作用于使用`{}`

```rust
fn main() {
    let y = {
        let x = 10;
        x + 20
    };
}
```

##### 将一个代码块绑定到变量y上。注意在`x+20`没有使用`;`的原因是，加上`;`这一段代码就变为了语句，将不会作为返回值。

#### 1. 什么是表达式？如何表达？

#### 2. 有哪些好处？

#### loop

```rust
fn main() {
    let mut counter = 10;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
}

```

### 其他问题

#### 包如何进行管理

#### 如何定义面向对象

#### 如何处理函数式编程

#### 如何进行多线程编程

#### 线程建如何进行通信
