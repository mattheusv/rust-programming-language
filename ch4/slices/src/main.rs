//https://doc.rust-lang.org/book/ch04-03-slices.html

fn main() {
    let s1 = String::from("Text Foo");
    let word = first_word(&s1);

    // s1.clear(); // compile error

    println!("the first word is: {}", word);

    let my_string_literal = "hello world";

    let word = first_word(my_string_literal);

    println!("the first word is: {}", word);

    let s2 = String::from("Hello World");
    let hello = &s2[0..5];
    let world = &s2[6..];

    println!("{} {}", hello, world);
}

fn first_word(s: &str) -> &str {
    let words = s.as_bytes();

    for (i, &word) in words.iter().enumerate() {
        if word == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
