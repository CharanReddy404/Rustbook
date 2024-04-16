fn main() {
    // another_function();
    // parameter_function(4);
    // print_labeled_measurement(24, 's', "hello");
    // expressions
    // let y = {
    //     let mut x = 3;
    //     x = x + 4;
    //     x * 2
    // };

    // println!("The value of y is: {y}");

    // let x = function_with_return(8);

    // println!("The value of x is: {x}");

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // Loop labels

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }

    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    // for loop
    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {element}");
    // }

    for i in (1..8).rev() {
        println!("{i}");
    }
}

// fn function_with_return(x: i32) -> i32 {
//     x * 5
// }

// fn another_function() {
//     println!("Another functions");
// }

// fn parameter_function(x: i32) {
//     println!("The value x is {x}")
// }

// fn print_labeled_measurement(value: i32, unit_label: char, word: &str) {
//     println!("The measurement is: {value} {unit_label} {word}");
// }
