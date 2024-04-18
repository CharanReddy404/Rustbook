#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        println!("{}----{}", self.area(), other.area());
        // self.area() > other.area()
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let sq = Rectangle::square(4);
    println!("{:#?}", sq);
    println!("area {}", sq.area());
    // dbg!(sq);

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
