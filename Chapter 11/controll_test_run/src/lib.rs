#![allow(unused)]

fn print_and_returns_10(a: i32) -> i32 {
    println!("i got the value {}", a);
    10
}

fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn add_two_inner(a: i32) -> i32 {
    add(a, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_adder() {
        assert_eq!(4, add_two_inner(2));
    }

    //ignore tests
    // cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // this code takes an hour to run
    }

    // running a subset of tests
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // cargo test -- --show-output
    #[test]
    fn this_test_will_pass() {
        let value = print_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = print_and_returns_10(5);
        assert_eq!(10, value);
        // assert_eq!(11, value);
    }

    // cargo test -- --test-threads=1
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two pluse two not equal to four"))
        }
    }

    #[test]
    fn it_works2() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
