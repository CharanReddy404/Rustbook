#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let config_max = Some(4u8);

    // // Using match
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    // Using if let

    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max)
    // }

    // println!("{:?}", config_max);

    // if let else
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);

    // Using Match
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // Using let if else
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
