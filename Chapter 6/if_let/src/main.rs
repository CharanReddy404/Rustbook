fn main() {
    println!("if let");

    let config_max = Some(8u8);

    // using if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("None")
    }
    // // using match
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
}
