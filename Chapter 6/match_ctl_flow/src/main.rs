// How Matches Interact with Ownership

fn main() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match &opt {
        Some(s) => println!("Some!"),
        None => println!("None!"),
    };

    println!("{:?}", opt);
}

// // Catch-all Patterns and the _ Placeholder

// fn main() {
//     // let dice_roll = 3;
//     // let dice_roll = 7;
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         // other => move_player(other),
//         // _ => reroll(),
//         _ => (),
//     }

//     fn add_fancy_hat() {
//         println!("add_fancy_hat");
//     }
//     fn remove_fancy_hat() {
//         println!("remove_fancy_hat");
//     }
//     // fn move_player(num_spaces: u8) {
//     //     println!("move_player {}", num_spaces);
//     // }
//     // fn reroll() {
//     //     println!("reroll");
//     // }
// }

// // Matching with Option<T>
// fn main() {
//     fn pluse_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = pluse_one(five);
//     let none = pluse_one(None);
// }

// -------------------------------------------------

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn main() {
//     println!("Matchs");
//     value_in_cents(Coin::Quarter(UsState::Alabama));
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky Penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quater from {:?}", state);
//             25
//         }
//     }
// }
