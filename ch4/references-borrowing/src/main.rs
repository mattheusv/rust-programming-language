// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s2 = String::from("Rust");

    println!("{}", s2);
    change(&mut s2);
    println!("{}", s2);

    // You can have only one mutable reference to a particular piece
    // of data in a particular scope.

    let mut s3 = String::from("multi mutables");

    {
        let s3r1 = &mut s3;
        println!("{}", s3r1);
    } // r1 goes out of scope here, so we can make a new reference
      // with no problems.

    let s3r2 = &mut s3;
    // let s3r3 = &mut s3; // compile error more than once at a time

    // println!("{}, {}", s3r2, s3r3);
    println!("{}", s3r2);


    let mut s4 = String::from("hello");

    let s4r1 = &s4;
    let s4r2 = &s4;
    // let s4r3 = &mut s4; // compile error: cannot borrow `s4`
    // as mutable because it is also borrowed as immutable

    println!("{}, {}", s4r1, s4r2);

    let mut s5 = String::from("hello");

    let s5r1 = &s5;
    let s5r2 = &s5;
    // r1 and r2 are no longer used after this point

    println!("{}, {}", s5r1, s5r2);

    let s5r3 = &mut s5; // no problem
    println!("{}", s5r3);

    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello");
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// compile error:
// this function's return type contains a borrowed value,
// but there is no value for it to be borrowed from.

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn calculate_length(s: &String) -> usize {
    return s.len()
} // Here, s goes out of scope. But because it does not have
  // ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", is awesome")
}
