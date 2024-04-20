fn main() {
    // // // unsafe
    // // read(x);
    // let x = true;
    // // // safe
    // // read(x);

    // let n = 5;
    // let y = plus_one(n);
    // println!("The value of y is: {y}");
    // println!("The value of y is: {n}");

    // let a = Box::new([2; 10]);
    // println!("a--{:?}", a);
    // // without clonng the a ownership will be moved so we should clone so we can use data of a later also
    // let b = a.clone();
    // println!("b--{:?}", b);
    // println!("a--{:?}", a);

    let s = String::from("hello");
    let mut _s2;
    _s2 = s;
    _s2 = String::from("Hello world");

    println!("{_s2}");
}

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// fn read(y: bool) {
//     if y {
//         print!("y is true");
//     }
// }
