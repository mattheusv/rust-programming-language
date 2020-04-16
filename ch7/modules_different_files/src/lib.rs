//https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html

mod front_of_house;

pub use crate::front_of_house::hosting;
pub use crate::front_of_house::service;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    front_of_house::some_fn();

    service::some_fn();
}
