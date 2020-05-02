//https://doc.rust-lang.org/book/ch08-01-vectors.html

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // use type anotation because init Vector without values
    let v: Vec<i32> = Vec::new();

    //macro to create a vector (without type anotation)
    let v2 = vec![1, 2, 3];

    //dont need type anotation because we pushing i32 values to vector
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);

    reading_elements();
    eight_seven();
    iterating_over_values();
    enum_store_multiple_types();
}

fn enum_store_multiple_types() -> Vec<SpreadsheetCell> {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    row
}

fn iterating_over_values() {
    let v = vec![100, 32, 57];

    //for loop to access immutable references
    for i in &v {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];

    for i in &mut v2 {
        *i += 50;
        println!("{}", i);
    }
}

fn reading_elements() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("Thre is no third element."),
    }
}

fn eight_seven() {
    // let mut v = vec![1, 2, 3, 4, 5];

    //compile error
    //because adding a new element to vector can cause
    //a deallocated to the pointer because a vector will need
    //allocate more memory
    //let first = &v[0];

    // v.push(6);

    // println!("The first element is: {}", first);
}
