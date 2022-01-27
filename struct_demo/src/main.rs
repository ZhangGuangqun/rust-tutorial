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

