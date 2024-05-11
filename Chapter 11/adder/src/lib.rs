#![allow(unused)]

use std::fmt::format;
#[derive(Debug)]

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Using Result<T, E> in Tests
    #[test]
    fn it_works_again() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 101.")]
    fn greeting_than_100() {
        Guess::new(101);
    }

    #[test]
    fn greeting_contains_name() {
        let name = "RustBook";
        let result = greeting(name);
        assert!(
            result.contains("RustBook"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 4,
            height: 6,
        };

        let smaller = Rectangle {
            width: 2,
            height: 3,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_works() {
        let a = 4;
        let b = 2;
        let c = 6;
        let result = add(a, b);
        assert_eq!(result, c, "{} + {} != {}", a, b, c);
    }

    // #[test]
    // fn failing_test() {
    //     panic!("Make this test fail");
    // }
}
