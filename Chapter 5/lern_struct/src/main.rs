#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{:#?}", user2);

    let black = Color(32, 34, 45);
    println!("{}---{}---{}", black.0, black.1, black.2);
}

// fn main() {
//     // let user = User {
//     //     active: true,
//     //     username: String::from("rusty"),
//     //     email: String::from("rusty@rust.com"),
//     //     sign_in_count: 1,
//     // };

//     // println!(
//     //     "----{}----{}----{}----{}----",
//     //     user.active, user.username, user.email, user.sign_in_count
//     // );

//     // -------------------------------------------------------------------------

//     // // To change the value inside struct we should mut it
//     // let mut user2 = User {
//     //     active: false,
//     //     username: String::from("rusty2"),
//     //     email: String::from("rusty2@rust.com"),
//     //     sign_in_count: 2,
//     // };

//     // println!(
//     //     "before----{}----{}----{}----{}----",
//     //     user2.active, user2.username, user2.email, user2.sign_in_count
//     // );

//     // user2.active = true;

//     // println!(
//     //     "after----{}----{}----{}----{}----",
//     //     user2.active, user2.username, user2.email, user2.sign_in_count
//     // );

//     // -------------------------------------------------------------------------

//     // let mut user1 = User {
//     let user1 = User {
//         active: true,
//         username: String::from("rusty"),
//         email: String::from("rusty@rust.com"),
//         sign_in_count: 1,
//     };

//     // let user2 = User {
//     //     active: user1.active,
//     //     username: user1.username,
//     //     email: String::from("anotherrusty2@rust.com"),
//     //     sign_in_count: user1.sign_in_count,
//     // };

//     // i convert the above code to this and it will still work
//     let user2 = User {
//         email: String::from("anotherrusty2@rust.com"),
//         ..user1
//     };

//     println!(
//         "user2----{}----{}----{}----{}----",
//         user2.active, user2.username, user2.email, user2.sign_in_count
//     );

//     // here if i try to access the user1.username i will get error because user1.username value has been moved to user2.username
//     // user1.username = String::from("username");
//     // println!(
//     //     "user1----{}----{}----{}----{}----",
//     //     user1.active, user1.username, user1.email, user1.sign_in_count
//     // );

//     // -------------------------------------------------------------------------
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }
