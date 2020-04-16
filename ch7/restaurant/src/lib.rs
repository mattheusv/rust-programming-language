//https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
#![allow(unused_variables)]
#![allow(unused_imports)]

//Nested imports
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;

// use std::io::{self, Write};


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitling() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        fn something_super() {
            take_order();
            super::hosting::add_to_waitling();
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,  // private field
    }

    // all fields are public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

fn serve_order() {}

pub fn eat_at_restaurant_use() {
    //Absolute path
    use crate::front_of_house::hosting;

    //Relative path
    // use front_of_house::hosting;

    hosting::add_to_waitling();
    hosting::add_to_waitling();
    hosting::add_to_waitling();

    //It's more idiomatic to specify full path when import
    //structs, enums adn other items
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(1, 2);


}

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitling();

    //Relative path
    front_of_house::hosting::add_to_waitling();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    //meal.seasonal_fruit = String::from("blueberries");  //compile error

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
