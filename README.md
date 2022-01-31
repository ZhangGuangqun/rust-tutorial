# rust tutorial

学习 rust 语言，按照官方文档把代码敲一遍

github 仓库：https://github.com/ZhangGuangqun/rust-tutorial

## 官方文档

https://www.rust-lang.org/zh-CN/learn/get-started

## hello world

https://doc.rust-lang.org/book/ch01-02-hello-world.html

``` rust
fn main() {
    // println! calls a Rust macro. If it called a function instead, it would be entered as println (without the !)
    println!("Hello, world!");
}
```

``` sh
rustc main.rs
./main
```

## cargo

``` sh
cargo new hello-rust

cargo run
```

``` sh
cargo check

cargo build
```

Building for Release

``` sh
cargo build --release
```

## guessing game

```
cargo new guessing_game
```

``` rust
let apples = 5; // 不可变变量 Variables 
let mut bananas = 5; // 可变变量 Mutability
```

const 常量


``` rust
let mut guess = String::new();
```

String::new() 创建一个新的空字符串，并且返回一个 String 类型的引用。

String 字符串

`::` 函数 (function) 调用

``` rust
io::stdin()
        .read_line(&mut guess)

// std::io::Stdin 实例 (instance)
```

`.` 方法 (method) 调用

`&` 引用

``` sh
// 抛出异常
.expect("Failed to read line");
```

``` toml
[dependencies]
rand = "0.8.3"
```

更新依赖

``` sh
cargo update
```

``` rust
use rand::Rng;

// 省略...

let secret_number = rand::thread_rng().gen_range(1..101);
```

The Rng `trait` defines methods that random number generators `implement`, and this trait `must` be in scope for us to use those methods

trait ? php ?

看到这里，个人感受：

各种语言都借用点？感觉没必要呀

```
c++ : `::` `use`
js: `let`
golang: `fn`

调用公共代码：

宏调用：`println!()`
函数 (function) 调用：`::`
方法 (method) 调用：`.`

```

1 到 100

```
1..101

1..=100
```

查看文档：

``` sh
cargo doc --open
```

rust switch 语法：

``` rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

match 里不用加 break

break 用在循环 loop 中退出

可以定义相同名称的变量，后面的可以覆盖前面的变量，常用作类型转换

``` rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

循环：

```
loop {
    // 退出循环，在适当的地方使用 break
}
```

错误处理：

``` rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

``` rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        println!("You guessed: {}", guess);
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

```

## Common Programming Concepts

### 变量、数据类型

``` sh
cargo new common_concepts
```

数值类型数字溢出问题

> When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

``` rust
fn main() {
    // 1. 变量 （Variables and Mutability）

    // 不可变变量 Variables 
    // 可变变量 Mutability
    // 如果要让变量可以重新赋值
    let mut x = 5;
    println!("x is {}", x);

    x = 6;
    println!("x is {}", x);

    // 常量 const
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

    println!("THREE_HOURS_IN_SECONDS is {}", THREE_HOURS_IN_SECONDS);


    let y = 1;

    // Shadowing 覆盖
    let y = y + 1;

    // 不可变变量 Variables 和 可变变量 Mutability 的区别
    // 不可变变量 Variables 在于需要改变时需要手动加 let , 并且在 let 之后都不可变
    // 不可变变量 Variables 可以改变类型

    {
        // 作用域 Scope
        let y = y * 2;
        println!("inner y is {}", y);
    }

    println!("y is {}", y);

    // 2. 数据类型 （Data Types）
    // guess 需要添加类型，否则编译器会报错
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess is {}", guess);

    // Scalar Types ： integers, floating-point numbers, Booleans, and characters
    
    let f1 = 1.0; // f64
    let f2 : f32 = 2.0; // f32

    println!("f1 is {}", f1);
    println!("f2 is {}", f2);

    // 数值运算

    // addition
    let sum = 5 + 10;

    println!("sum is {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;

    println!("difference is {}", difference);

    // multiplication
    let product = 4 * 30;

    println!("product is {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    println!("quotient is {}", quotient);
    println!("floored is {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("remainder is {}", remainder);

    let t = true;
    println!("t is {}", t);

    let f: bool = false; // with explicit type annotation
    println!("f is {}", f);

    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat is {}", heart_eyed_cat);

    // Compound Types 组合类型
    // Tuples

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("The value of y is: {}", y);

    println!("The tuple first value is: {}", tup.0);

    // Arrays 数组
    let a = [1, 2, 3, 4, 5];
    println!("a length is {}", a.len());

    // 指定 Array 的类型和长度
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a length is {}", a.len());

    // 指定数组的初始值和长度
    let a = [3; 5];
    println!("a first element value is {}", a[0]);
}
```

### 函数

``` sh
cargo new functions
```

``` rust
fn main() {
    println!("Hello, world!");
    another_function();

    add(1, 2);

    // 表达式赋值
    let y = {
        let x = 3;
        x + 1 //不能加分号
    };

    println!("The value of y is: {}", y);

    let result = add_return(1, 2);
    println!("The result is: {}", result);
}

// 函数名小写，用下划线分割，定义的顺序无所谓，不需要在要调用之前声明
fn another_function() {
    println!("Another function.");
}

// 带参数的函数
fn add(x: i32, y: i32) {
    println!("x + y = {}", x + y);
}

fn add_return(x: i32, y: i32) -> i32 {
    x + y
}
```


### 流程控制

``` sh
cargo new flow_control
```

``` rust
fn main() {
    // 1. if
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    // 类似三目表达式
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // 2. 循环 loop , while , for
    
    // loop {
    //     println!("again!");
    // }

    let mut count = 0;

    // 可以对 loop 加标签，在 break 时可以指定标签来退出循环
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {}", result);

    // while
    let mut counter = 1;

    while counter <= 10 {
        println!("counter = {}", counter);
        counter += 1;    
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // 1..4 Range  (1..4).rev() 倒序
    for number in 1..4 {
        println!("{}!", number);
    }
}

```

## Ownership

> Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector

> memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

在编译的时候，编译器会检测代码需要遵循一系列规则，如果违反了一些规则，编译会报错


- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.


``` sh
cargo new ownership
```

``` rust
fn main() {
    let mut s = String::from("hello");
    // let mut s = "hello world";

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1.clone();  // let s2 = s1; 这样写 s1 会 move to s2, 因此 s1 的内存会被释放

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    // x 不会被 move
    // Rust has a special annotation called the trait that we can place on types that are stored on the stack like integers are.
    //  If a type implements the trait, a variable is still valid after assignment to another variable.
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // println!("s is {}", s);   // s 被 move 了再调用 takes_ownership 后，因此被销毁了，不能使用了

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward


    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

```

### references and borrowing

```
cargo new references_borrowing
```

``` rust
fn main() {
    let mut s = String::from("hello world");

    let str_length = calculate_length(&s);

    println!("The length of '{}' is {}.", s, str_length);

    change(&mut s);
    change(&mut s);

    // 同一时间只能有一个可变引用
    // 同一时间也不能同时出现多个不可变引用及可变引用
    // let s1 = &mut s;
    // let s2 = &mut s;
    // println!("s1: {}, s2: {}", s1, s2);

    println!("changed s: {}", s);
}

// 引用不会拿去所有权 （don't taking ownership of the value）
// 引用使用操作符 &
// Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *
fn calculate_length(s: &String) -> usize {
    // s.push_str("test");  // 引用的是一个不可变的变量，所以不能改变它的值
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" test ");
}

// Dangling References
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

```

### slice 切片

> Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

slice 是一种引用，因此它没有所有权

``` sh
cargo new slice
```

``` rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let first_world = first_word(&s);
    println!("{}", first_world);

    // s.clear();  // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // 可变引用和不可变引用不能在同一时间内出现

    println!("{}", first_world);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

```

## struct

``` sh
cargo new struct_demo
```

``` rust
struct User {
    username: String,
    email: String,
    age: u32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut u1 = User {
        username: String::from("some_username"),
        email: String::from("test@test.com"),
        age: 18, 
    };
    println!("{}", u1.username);

    u1.username = String::from("new_username");
    println!("{}", u1.username);

    let u2 = User {
        username: String::from("zhangsan"),
        email: String::from("test1@test.com"),
        age: u1.age,
    };

    println!("{}", u2.username);

    let u3 = User {
        username: String::from("lisi"),
        ..u2
    };
    println!("{}", u3.username);


    // You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to ()
    // Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}

```

### method

``` rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 ares is {}", rect1.area());
}

```

## enum

``` sh
cargo new enums
```

``` rust
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("enum method")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    println!("{:?}", four);

    let local_address = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:?}", local_address);

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    println!("{:?}", home);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let whichKind = match four {
        IpAddrKind::V4 => 4,
        IpAddrKind::V6 => 6
    };
    println!("{}", whichKind);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 使用 other 来覆盖其他的情况，如果不需要使用值可以用 _ 来代替, 不执行操作可以使用: _ => {}
        other => move_player(other),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

```

## packages 、crates 、modules

``` sh
cargo new --lib restaurant
```
