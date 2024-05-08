#![allow(unused)]

struct MPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> MPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MPoint<X2, Y2>) -> MPoint<X1, Y2> {
        MPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = MPoint { x: 5, y: 10.4 };
    let p2 = MPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// --------------------------------------------
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct BPoint<T, U> {
    x: T,
    y: U,
}

fn old3_main() {
    let both_integer = BPoint { x: 5, y: 10 };
    let both_float = BPoint { x: 1.0, y: 4.0 };
    let integer_and_float = BPoint { x: 5, y: 4.0 };

    // single type
    let integer = Point { x: 5, y: 10 };
    println!("integer.x = {}", integer.x());
    let float = Point { x: 1.0, y: 4.0 };
    println!("float.x = {}", float.x());
    println!(
        "float.distance_from_origin = {}",
        float.distance_from_origin()
    );

    // // eroor code
    // let wont_work = Point { x: 5, y: 4.0 };
}

fn old2_main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// without using generics
fn old_main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
