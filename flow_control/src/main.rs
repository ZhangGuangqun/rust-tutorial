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
