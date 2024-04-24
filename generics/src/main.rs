fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, U> Point<T, U> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl<T> Summary for Point<f64, T> {}

impl Summary for Point<i32, i32> {
    fn summarize(&self) -> String {
        format!("{} {}", self.x, self.y)
    }
}

pub fn notify(item: &impl Summary) {
    println!("summaries {}", item.summarize());
}

fn longest<'a>(x: &'a str, y:&str) -> &'a str {
    x
}

fn violation() {
    let s1 = String::from("123");
    {
        let s2 = String::from("234");
        let result;
        result = longest(&s1, &s2);
        println!("The longest is {}", result);
    }
}

fn main() {
    let number_list = vec![1, 2, 3];
    println!("The largest number is {}", largest(&number_list));
    let char_list = vec!['h', 'l'];
    println!("The largest char is {}", largest(&char_list));

    let point = Point {x: 5, y: 4};
    dbg!(point.summarize());
    let point1 = Point {x: 0.5, y: "what"};

    let p3 = point1.mixup(point);
    dbg!(p3.summarize());

    notify(&p3);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = longest(&s1, &s2);

    violation();
}