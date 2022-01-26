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
