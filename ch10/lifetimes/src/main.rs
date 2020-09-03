// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html


fn main() {
    let s1 = String::from("long string");

    {
        let s2 = String::from("short");
        let result = longest(s1.as_str(), s2.as_str());
        println!("{}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Invalid code because the compiler dont't know
// which reference will be returned, x or y.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
