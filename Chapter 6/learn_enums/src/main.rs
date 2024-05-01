#[derive(Debug)]
// implement using enum
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// // implement using stuct
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    //implemt using enum
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    // // implement using stuct
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
}

// #[derive(Debug)]

// // code using struct
// // enum IpAddrKind {
// //     V4,
// //     V6,
// // }

// // struct IpAddr {
// //     kind: IpAddrKind,
// //     address: String,
// // }

// // let home  = IpAddr{
// //     kind: IpAddrKind::V4,
// //     address:String::from("127.0.0.1"),
// // }

// // let loopvack  = IpAddr{
// //     kind: IpAddrKind::V6,
// //     address:String::from("::1"),
// // }

// // code implement using enum
// // enum IpAddr {
// //     V4(String),
// //     V6(String),
// // }

// enum IpAddr {
//     V4(u8,u8,u8,u8),
//     V6(String),
// }

// let home = IpAddr::V4(127,0,0,1);
// let lookback = IpAddr::V6(String::from("::1"));

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     dbg!(four);
//     dbg!(six);

//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }

// fn route(ip_kind: IpAddrKind) {}

// ------------------------------------------------------------
// another example

// using struct
// struct QuitMessage; //unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); //tuple struct
// struct ChangeColorMessage(i32, i32, i32); //tuple struct

// // using enum
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn call(&self) {
//         // body of code
//         dbg!(self);
//     }
// }

// fn main() {
//     let m = Message::Write(String::from("Hello"));
//     m.call();
// }
