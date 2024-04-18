// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// Method Syntax
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    // Method syntax
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("The rectangle has a nonzero width; it is {:#?}", rect1);
    // // version 1
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels",
    //     area(width1, height1)
    // );

    // // Version 2 - using tuples
    // let rect1 = (30, 50);
    // println!("The area of the rectangle is {} square pixels", area(rect1));

    // // Version 3 - using struct
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // println!("rect1 is {:#?}", rect1);
    // println!(
    //     "The area of the rectangle is {} square pixels",
    //     area(&rect1)
    // );
    // // println!("{}", rect1.height)

    // // dbg!
    // let scale = 2;
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    // dbg!(&rect1);
    // println!("{:#?}", rect1);

    // Method Syntax
}

// // Version 3 - using struct
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// // Version 2 - using tuples
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// //  Version 1
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
