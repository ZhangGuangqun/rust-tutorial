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
