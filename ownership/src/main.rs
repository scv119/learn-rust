fn main() {
    let s = String::from("hello");
    let s1 = s.clone();
    
    // take_ownership(s);

    println!("{}", s);

    let x = 5;

    make_copy(x);

    println!("{}", x);
}

fn take_ownership(str: String) {
    println!("{}", str);
}

fn make_copy(int: i32) {
    println!("{}", int);
}
