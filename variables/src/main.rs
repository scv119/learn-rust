use std::io;

const MAX_POINTS: u32 = 3 * 100_000;

fn another_function(input: i32) {
    println!("Another function. {}", input);
}

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    {
        let x = 7;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let sum = 3 + 5;
    let fnumber = 3.0;
    let f1: f32 = 2.0;

    let t = true;
    let f: bool = false;

    let tup = (500, 6.4, 1);
    let five_hunderd = tup.0;

    expression();
    let x = plus_one(5);
    println!("The value of x is: {}", x);

    if_statement(3);
    if_statement(7);

    println!("The value of if_statement2 is: {}", if_statement2());

    loop_function();
    loop_function1();
}

fn read_index() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let mut index = String::new();

    println!("Please enter an index.");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to readline");

    let index: usize = index.trim().parse().expect("Index must be a number");

    let element = a[index];

    println!("The value of element is: {}", element);

    loop_function();
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> u32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn if_statement(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_statement2() -> i32 {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    number
}

fn loop_function() {
    let a = [1, 2, 3, 4, 5];
    for number in a.iter().rev() {
        println!("The value is: {}", number);
    }
}

fn loop_function1() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}
