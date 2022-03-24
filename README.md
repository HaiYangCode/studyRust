## Rust区别于其他语言的特点

#### Mac环境下安装及更新，卸载

```shell
#mac 环境下安装rust环境
curl https://sh.rustup.rs -sSf | sh #下载rust安装脚本
rustup update #更新rust环境
rustup self uninstall #卸载rust环境
```

### 项目结构介绍

#### cargo介绍

##### 1. 文件夹目录结构

##### 2. Cargo.toml作用及使用

##### 3. Cargo.lock作用及使用
该文件是自动生成，记录项目中依赖的三方库信息，名字，版本
##### cargo相关命令
```shell
cargo --version # 查看版本号
cargo build #编译项目，在根目录下的target/debug/项目名；找到执行文件。
cargo build --release #构建优化模式下的程序
cargo run #直接运行项目
cargo run --release #运行优化模式下运行程序
cargo check #校验代码的正确性
cargo new projectName #创建一个工程名为projectName
```


#### 基本语法特点

#### 函数定义

##### 1. main的定义及特点

###### 1. 定义
fn 定义一个函数
```rust
fn main() {
    println!("hello world");
}
```

###### 2. 函数中知识点

1. Rust风格使用4个空格键而不是tab实现缩进。
2. 调用了一个叫`println！`的宏，如果是普通函数需要去掉`!`号。 
* 宏调用和普通函数调用有什么区别？
4. 使用`;`作为一行代码的结尾

###### 2. 有参数怎么获取？

`main`函数默认没有参数,如果有参数的话需要将他们放置在括号`()`中。

#### rustfmt的作用及使用

用于格式化rust代码。

#### 在命令行中运行一个rust程序

```shell
rustc main.rs # 运行简单的rust程序
```

#### `crate`和`package`的区别
`crate`是Rust中的最小的编译单元，`package`是单个或多个crate的集合，因为单个`crate`也是一个`package`，
但`package`通常倾向于多个crate的组合，本书中`crate`和`package`统一被翻译为包，只在两者同事出现且需要区别对待时，
将`crate`译为单元包，将`package`译为包。


#### use 作用
导入非预导入的标准库