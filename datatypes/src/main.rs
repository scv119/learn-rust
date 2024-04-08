#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn double(&mut self)  {
        self.width *= 2;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
}


fn main() {
    let mut rect1 = Rectangle {
        width: dbg!(30 * 5),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    dbg!(rect1.area());
    rect1.double();
    dbg!(rect1.area());
    println!("can hold {}",rect1.can_hold(&rect2));
    println!("can hold {}",rect2.can_hold(&rect1));
    dbg!(Rectangle::square(1));
}
