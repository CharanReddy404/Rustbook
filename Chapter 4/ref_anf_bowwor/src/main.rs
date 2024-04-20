fn main() {
    // let mut v: Vec<i32> = vec![1, 2, 3];
    // println!("{:?}", v);
    // v.push(4);
    // println!("{:?}", v);

    // error code
    // let mut v: Vec<i32> = vec![1, 2, 3];
    // let num: &i32 = &v[2];
    // v.push(4);
    // println!("Third element is {}", *num);
}

// // Dereferencing a Pointer Accesses Its Data
// fn main() {
//     let mut x: Box<i32> = Box::new(1);
//     let a: i32 = *x; // *x reads the heap value, so a = 1
//     *x += 1; // *x on the left-side modifies the heap value,
//              //     so x points to the value 2

//     let r1: &Box<i32> = &x; // r1 points to x on the stack
//     let b: i32 = **r1; // two dereferences get us to the heap value

//     let r2: &i32 = &*x; // r2 points to the heap value directly
//     let c: i32 = *r2; // so only one dereference is needed to read it
// }

// // References Are Non-Owning Pointers
// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(&m1, &m2); // note the ampersands
//     let s = format!("{} {}", m1, m2);
//     println!("{}!", s);
//     // println!("{} {}!", m1, m2);
// }

// fn greet(g1: &String, g2: &String) {
//     // note the ampersands
//     println!("{} {}!", g1, g2);
// }

// // fn main() {
// //     let m1 = String::from("Hello");
// //     let m2 = String::from("world");
// //     let (m1_again, m2_again) = greet(m1, m2);
// //     let _s = format!("{} {}", m1_again, m2_again);
// //     println!("{} --- {}", m1_again, m2_again);
// //     // println!("{} --- {}", m1, m2);
// // }

// // fn greet(g1: String, g2: String) -> (String, String) {
// //     println!("{} {}!", g1, g2);
// //     (g1, g2)
// // }
