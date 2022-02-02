fn main() {
    let v1 : Vec<i32> = Vec::new();

    let mut v2 = vec![1, 2, 3];
    
    let first = &v2[0];

    v2.push(4);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    // println!("The first element is {}", first); // error[E0502]: cannot borrow `v2` as mutable because it is also borrowed as immutable

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        //  dereference operator (*) to get to the value in i
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // 使用 vector 存储不同类型的元素
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
