fn main() {
    // let f_to_c = fahrenheit_to_celsius(32.0);
    // println!("{f_to_c}");

    // let c_to_f = celsius_to_fahrenheit(0.0);
    // println!("{c_to_f}")
    // let n = nth_fibonacci_number(10);
    // println!("fibonacci_number {n}")

    print_the_lyrics();
}

// fn fahrenheit_to_celsius(f: f32) -> f32 {
//     (f - 32.0) * 5.0 / 9.0
// }

// fn celsius_to_fahrenheit(c: f32) -> f32 {
//     c * 9.0 / 5.0 + 32.0
// }

// fn nth_fibonacci_number(n: i32) -> i32 {
//     if n == 0 {
//         return 0;
//     }

//     if n == 1 {
//         return 1;
//     }

//     let mut count = 1;
//     let mut f1 = 0;
//     let mut f2 = 1;
//     let mut f_no = f1 + f2;

//     while count < n {
//         f_no = f1 + f2;
//         f1 = f2;
//         f2 = f_no;
//         count += 1;
//     }

//     return f_no;
// }

fn print_the_lyrics() {
    let days: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "FIVE GOLD RINGS",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 0..12 {
        let suffex: &str = match day + 1 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!(
            "\nOn the {}{} day of xmas my true love gave to me:",
            day + 1,
            suffex
        );
        for item in (0..day + 1).rev() {
            if day > 0 && item == 0 {
                print!("Add ");
            }
            println!("{}", days[item]);
        }
    }
}
