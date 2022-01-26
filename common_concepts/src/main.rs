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
