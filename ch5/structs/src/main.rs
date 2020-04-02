//https://doc.rust-lang.org/book/ch05-01-defining-structs.html

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("foobar@gmail.com"),
        username: String::from("baz"),
        active: true,
        sign_in_count: 1,
    };
    println!("{} {}", user1.email, user1.username);

    let mut user2 = User {
        email: String::from("foobar2@gmail.com"),
        username: String::from("baz2"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("newemail@bla.com");

    let user3 = build_user(
        String::from("gmail@email.com"), String::from("hello")
    );

    println!("{} {}", user3.email, user3.username);

    let user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // Fill rest of filds with the values of user1 instance
    };

    println!("{} {}", user4.active, user4.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}, {}, {}", black.0, black.1, black.2);
    println!("{}, {}, {}", origin.0, origin.1, origin.2);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
