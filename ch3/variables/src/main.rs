fn main() {
    let x = 5;
    let mut x_mut = 5;
    println!("The value of x is: {}", x);
    println!("The value of x_mut is: {}", x_mut);
    // compile error
    // cannot assign twice to immutable variable `x`
    // x = 6;
    x_mut = 6;
    println!("The value of x is: {}", x);
    println!("The value of x_mut is: {}", x_mut);

    //Underscores can be inserted in numeric literals to improve readability
    const MAX_POINTS: u32 = 100_000;
    println!("Constant value: {}", MAX_POINTS);

    //Shadowing

    let a = 10;

    let a = a + 1;

    let a = a * 2;

    println!("The value of a is: {}", a);

    let some_str = "some random string";
    println!("Before shadowing str: {}", some_str);

    let some_str = "shadowing some_str";

    println!("After shadowing str: {}", some_str);

    // change type of variable

    let spaces = "     ";

    let spaces = spaces.len();

    println!("spaces: {}", spaces);
}
