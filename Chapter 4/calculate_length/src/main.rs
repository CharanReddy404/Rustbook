fn main() {
    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(&s1);
    // let len = calculate_length(&s1);
    // println!("the length of {} is {}", s1, len);

    // let s1 = String::from("hello");

    // let r1 = &s1;
    // let r2 = &s1;

    // println!("{}----{}", r1, r2);
    // println!("{}----{}", r1, r2);

    // let reference_to_nothing = dongle(); //error

    // slice

    // let mut s = String::from("Hello World");
    // let word = first_word(&s);
    // println!("{} ---- {}", s, word);
    // s.clear();
    // println!("{} ---- {}", s, word);

    // let mut s = String::from("Hello World");
    // let hello = &s[..5];
    // let world = &s[..];

    // let word = first_word(&s);
    // println!("{} ---- {} --- {}", hello, world, word);
    // s.clear();
    // println!("{}", word);

    // let mut s = String::from("Hello World");
    // let _s2 = "hello world";

    // let word = first_word(&s);
    // println!("{}", word);
    // s.clear();
    // println!("{}", s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn dongle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn calculate_length(s: &String) -> usize {
//     let length = s.len();
//     length
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }
