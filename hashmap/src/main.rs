use std::collections::HashMap;

fn main() {
    let field_name = String::from("AA");
    let field_value = String::from("BB");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("field_name: {}", field_name); // error[E0382]: borrow of moved value: `field_name`

    let field_value_in_map = map.get(&String::from("AA"));
    println!("field_value_in_map: {}", field_value_in_map.unwrap());

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 更新 HashMap 里的值
    map.insert(String::from("AA"), String::from("CC"));
    println!("map: {:?}", map);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // 检查对应的键是否存在
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut update_map = HashMap::new();

    for word in text.split_whitespace() {
        // 拿到对应键值的引用，然后基于引用更新
        let count = update_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", update_map);
}
