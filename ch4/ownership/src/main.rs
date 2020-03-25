fn main() {
    let mut foo = "foo";

    let mut bar = String::from("bar");

    println!("{}", foo);
    println!("{}", bar);

    foo = "fuz";
    bar = String::from("baz");

    println!("{}", foo);
    println!("{}", bar);

    let mut s1 = String::from("s1");
    let s2 = s1.clone();

    println!("s1: {}", s1);
    println!("s2: {}", s2);

    s1 = String::from("s2");

    println!("s1: {}", s1);
    println!("s2: {}", s2);

    let mut a = "a";
    let b = a;

    println!("a = {}", a);
    println!("b = {}", b);

    a = "c";

    println!("a = {}", a);
    println!("b = {}", b);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    // println!("{}", s);  //compile borrow moved error

    let x = 5; // x comes into scope

    makes_copy(x);

    println!("{}", x);

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
                                       // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
                                       // moved, so nothing happens. s1 goes out of scope and is dropped.

    println!("s1 = {}", s1);
    // println!("s2 = {}", s2); // compile borrow error
    println!("s3 = {}", s3);

    let s10 = String::from("hello");

    let (s11, length) = calculate_length(s10);

    println!("{}: {}", s11, length);
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}
