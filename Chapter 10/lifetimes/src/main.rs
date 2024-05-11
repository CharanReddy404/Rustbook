#![allow(unused)]

// Generic Type Parameters, Trait Bounds, and Lifetimes Together

use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {}", result);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// The Static Lifetime

fn main_static() {
    let s: &'static str = "I have a static lifetime.";
}

// Lifetime Annotations in Method Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        &self.part
    }
}

fn main_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Generic Lifetimes in Functions
// Lifetime Annotation Syntax
// Lifetime Annotations in Function Signatures
// Lifetime Annotations in Struct Definitions
fn old_main2() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fix for below programe
fn old_main1() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}

// // we will gat an error because we are trying to acces the out of scope varible
// fn old_main() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r:{}", r);
// }
