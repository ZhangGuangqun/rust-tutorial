mod front_of_house;

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 使用 super 来访问父级模块
        super::serve_order();
    }

    fn cook_order() {}


    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// 使用 use 将模块引入到当前作用域
use crate::front_of_house::hosting;

// 可以使用相对路径来引入模块
// use self::front_of_house::hosting;

// 将方法引入
// use crate::front_of_house::hosting::add_to_waitlist;

// 使用 pub use 将模块中 re-exporting
// pub use crate::front_of_house::hosting;

// 缩写
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;

// use std::io::{self, Write};

// use std::collections::*;

use std::fmt::Result;
// 使用 as 来取别名
use std::io::Result as IoResult;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // 引入模块后，可以使用模块名作为前缀来访问模块中的函数
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
