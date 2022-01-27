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
