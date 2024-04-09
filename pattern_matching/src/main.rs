use std::ops::Add;

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32), 
}

impl Message {
    fn call(&self) {
        dbg!(self);
    }
}

#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

impl<T: std::fmt::Display> Option<T> {
    fn unwrap(&self) -> &T {
        match self {
            Option::Some(value) => value,
            Option::None => panic!("None value"),
        }
    }

    fn print(&self) {
        if let Option::Some(value) = self {
            println!("value is {}", value);
        } else {
            println!("value is None");
        }
    }
}

fn main() {
    let home: IpAddr = IpAddr::V4(127, 0, 0, 1);
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
    dbg!(&home);
    dbg!(&loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Option::Some(5);
    let none_number: Option<usize> = Option::None;

    println!("{:?}", some_number.unwrap());
    some_number.print();
    none_number.print();
}