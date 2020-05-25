use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    let mut data = HashMap::new();

    data.insert(1, String::from("data"));
    data.insert(5, String::from("data"));


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores2: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();


    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // For types that implement the Copy trait, like i32
    // the values are copied into the hash map.
    // For owned values like String, the values will be
    // moved and the hash map will be the owner of those values.
    //
    // If we insert references to values into the hash map
    // the values won’t be moved into the hash map.
    // The values that the references point to must be valid
    // for at least as long as the hash map is valid.

    let time = "Blue";
    let score = scores.get(time);

    match score {
        None => println!("Invalid time: {}", time),
        Some(i) => println!("{}", i),
    }

    let mut scores3 = HashMap::new();

    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Blue"), 25);

    // The original value of 10 has been overwritten.
    println!("{:?}", scores3);

    scores3.entry(String::from("Yellow")).or_insert(50);
    scores3.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores3);

    let text = "hello world wonderful world";

    let mut map4 = HashMap::new();

    for word in text.split_whitespace() {
        // count is mutable reference to the value of key
        let count = map4.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map4);
}
