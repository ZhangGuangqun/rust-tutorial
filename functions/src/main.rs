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
