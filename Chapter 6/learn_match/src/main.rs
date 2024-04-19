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
fn main() {
    // value_incents(Coin::Penny);
    // value_incents(Coin::Quarter(UsState::Alabama));

    // let five = Some(5);
    // let six = pluse_one(five);
    // let none = pluse_one(None);

    // println!("{:#?} --- {:#?} --- {:#?}", five, six, none);

    let dice_roll = 1;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn add_fancy_hat() {
    println!("add_fancy_hat");
}
fn remove_fancy_hat() {
    println!("remove_fancy_hat");
}
fn reroll() {
    println!("reroll");
}

// fn value_incents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// fn pluse_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
