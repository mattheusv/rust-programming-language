fn main() {
    //https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let plus_result = plus_one(10);

    println!("The value of plus_result is: {}", plus_result);
}

fn plus_one(y: i32) -> i32 {
    // last expression without semi colon is return of the function
    y + 1
}
