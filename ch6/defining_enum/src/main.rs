
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr::V4(127, 0, 0, 1);

    let loopback2 = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    // let absent_number = None; // compile error

    let some = some_fn();

    if some.is_some() {
        let value = some.unwrap();
        println!("{}", value);
    } else {
        println!("None");
    }

}

fn route(ip_kind: IpAddrKind) {}

fn some_fn() -> Option<String> {
    Some(String::from("some"))
    // None
}
