//https://doc.rust-lang.org/book/ch08-02-strings.html

fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s2 = String::from("hello");
    let s3 = String::from(" world");
    let s4 = s2 + &s3;
    println!("{}", s4);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);


    let s1 = String::from("hello");
    //let h = s1[0]; //compile error, Rust strings don’t support indexing

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
