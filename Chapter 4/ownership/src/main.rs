fn main() {
    let x = 5;
    let _y = x; // Copy

    // let s1 = String::from("hello");
    // let s2 = s1; // Move not a shallow copy

    // println!("{s1} = {s2}") // Error: value borrowed here after move

    // This will work because we are cloning
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("{s1} = {s2}")

    // // String are moved so we will get error
    // let s: String = String::from("some String");
    // take_ownership(s);
    // println!("{}", s);

    // // integer are copied so we will not get error
    // let i: i32 = 34;
    // make_copy(i);
    // println!("{}", i);

    // String giving_ownership
    let s1 = giving_ownership();
    let s2 = String::from("take_and_give_back");
    let s3 = take_and_give_back(s2);
    println!("---{}------{}---", s1, s3);
}

// fn take_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn make_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

fn giving_ownership() -> String {
    let some_string = String::from("giving_ownership");
    some_string
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}
